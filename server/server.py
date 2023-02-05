import asyncio
import logging
import argparse

import grpc
import backend_pb2
import backend_pb2_grpc

from transformers import AutoModelForCausalLM, AutoTokenizer


class TaskRunner(backend_pb2_grpc.TaskServicer):
    def __init__(self, models) -> None:
        super().__init__()

        self._locks = {}
        self._tokenizers = {}
        self._models = {}

        for model in models:
            logging.info("Loading %s tokenizer" % model)
            self._tokenizers[model] = AutoTokenizer.from_pretrained(model)

            logging.info("Loading %s model" % model)
            self._models[model] = AutoModelForCausalLM.from_pretrained(model)

            logging.info("Creating lock for %s" % model)
            self._locks[model] = asyncio.Lock()

    async def LoadModel(
        self,
        request: backend_pb2.LoadModelRequest,
        context: grpc.aio.ServicerContext
    ) -> backend_pb2.LoadModelReply:

        if self._locks[request.model] == None:
            logging.info("Loading %s tokenizer" % request.model)
            self._tokenizers[request.model] = AutoTokenizer.from_pretrained(
                request.model)

            logging.info("Loading %s model" % request.model)
            self._models[request.model] = AutoModelForCausalLM.from_pretrained(
                request.model)

            logging.info("Creating lock for %s" % request.model)
            self._locks[request.model] = asyncio.Lock()

        return backend_pb2.LoadModelReply(success=True)

    async def UnloadModel(
        self,
        request: backend_pb2.LoadModelRequest,
        context: grpc.aio.ServicerContext
    ) -> backend_pb2.LoadModelReply:

        logging.info("Unloading %s" % request.model)
        self._locks[request.model] = None
        self._tokenizers[request.model] = None
        self._models[request.model] = None

        return backend_pb2.LoadModelReply(success=True)

    async def RunTask(
        self,
        request: backend_pb2.TaskRequest,
        context: grpc.aio.ServicerContext
    ) -> backend_pb2.TaskReply:

        if self._locks[request.model] == None:
            context.set_code(grpc.StatusCode.FAILED_PRECONDITION)
            context.set_details("Model not loaded")
            return backend_pb2.TaskReply()

        reply = backend_pb2.TaskReply(data=[""])

        await self._locks[request.model].acquire()
        try:
            inputs = self._tokenizers[request.model](
                request.input, return_tensors="pt")
            tokens_in = len(inputs["input_ids"][0])
            if request.token_window > 0:
                tokens_in = min(tokens_in, request.token_window)

            outputs = self._models[request.model].generate(inputs["input_ids"][:, -request.token_window:],
                                                           min_length=tokens_in + request.min_length,
                                                           max_length=tokens_in + request.max_length,
                                                           do_sample=request.do_sample,
                                                           early_stopping=request.early_stopping,
                                                           top_p=request.top_p,
                                                           top_k=request.top_k,
                                                           temperature=request.temperature,
                                                           repetition_penalty=request.repetition_penalty,
                                                           length_penalty=request.length_penalty,
                                                           num_beams=request.num_beams,
                                                           num_beam_groups=request.num_beam_groups,
                                                           num_return_sequences=request.num_return_sequences,
                                                           no_repeat_ngram_size=request.no_repeat_ngram_size,
                                                           )

            results = []
            for output in outputs:
                results.append(self._tokenizers[request.model].decode(
                    output[tokens_in:], skip_special_tokens=True))

            reply = backend_pb2.TaskReply(data=results)
        finally:
            self._locks[request.model].release()

        return reply


async def serve(listen_addr, models) -> None:
    server = grpc.aio.server()

    backend_pb2_grpc.add_TaskServicer_to_server(
        TaskRunner(models), server)

    server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)

    await server.start()
    await server.wait_for_termination()

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(
        prog='RPC Transformer Server',
        description='Serves Transformer models over RPC to save resources'
    )

    parser.add_argument('listen_addr')
    parser.add_argument("models", nargs='+')

    args = parser.parse_args()

    asyncio.run(serve(args.listen_addr, args.models))
