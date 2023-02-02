import asyncio
import logging
import argparse

import grpc
import backend_pb2
import backend_pb2_grpc

from transformers import AutoModelForCausalLM, AutoTokenizer


class TaskRunner(backend_pb2_grpc.TaskServicer):
    def __init__(self, model_name: str) -> None:
        super().__init__()

        self._transformer_lock = asyncio.Lock()

        logging.info("Loading tokenizer")
        self._tokenizer = AutoTokenizer.from_pretrained(model_name)

        logging.info("Loading model")
        self._model = AutoModelForCausalLM.from_pretrained(model_name)

    async def RunTask(
        self,
        request: backend_pb2.TaskRequest,
        context: grpc.aio.ServicerContext
    ) -> backend_pb2.TaskReply:

        reply = backend_pb2.TaskReply(data=[""])

        await self._transformer_lock.acquire()
        try:
            inputs = self._tokenizer(request.input, return_tensors="pt")
            tokens_in = len(inputs["input_ids"][0])
            if request.token_window > 0:
                tokens_in = min(tokens_in, request.token_window)

            outputs = self._model.generate(inputs["input_ids"][:, -request.token_window:],
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
                results.append(self._tokenizer.decode(
                    output[tokens_in:], skip_special_tokens=True))

            reply = backend_pb2.TaskReply(data=results)
        finally:
            self._transformer_lock.release()

        return reply


async def serve(listen_addr, model) -> None:
    server = grpc.aio.server()

    backend_pb2_grpc.add_TaskServicer_to_server(
        TaskRunner(model), server)

    server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)

    await server.start()
    await server.wait_for_termination()

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(
        prog='RPC Transformer Server',
        description='Serves a Transformer model over RPC to save resources'
    )

    parser.add_argument('listen_addr')
    parser.add_argument('model')

    args = parser.parse_args()

    asyncio.run(serve(args.listen_addr, args.model))
