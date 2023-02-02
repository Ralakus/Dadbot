from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class TaskReply(_message.Message):
    __slots__ = ["data"]
    DATA_FIELD_NUMBER: _ClassVar[int]
    data: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, data: _Optional[_Iterable[str]] = ...) -> None: ...

class TaskRequest(_message.Message):
    __slots__ = ["do_sample", "early_stopping", "input", "length_penalty", "max_length", "min_length", "model", "no_repeat_ngram_size", "num_beam_groups", "num_beams", "num_return_sequences", "repetition_penalty", "temperature", "token_window", "top_k", "top_p"]
    DO_SAMPLE_FIELD_NUMBER: _ClassVar[int]
    EARLY_STOPPING_FIELD_NUMBER: _ClassVar[int]
    INPUT_FIELD_NUMBER: _ClassVar[int]
    LENGTH_PENALTY_FIELD_NUMBER: _ClassVar[int]
    MAX_LENGTH_FIELD_NUMBER: _ClassVar[int]
    MIN_LENGTH_FIELD_NUMBER: _ClassVar[int]
    MODEL_FIELD_NUMBER: _ClassVar[int]
    NO_REPEAT_NGRAM_SIZE_FIELD_NUMBER: _ClassVar[int]
    NUM_BEAMS_FIELD_NUMBER: _ClassVar[int]
    NUM_BEAM_GROUPS_FIELD_NUMBER: _ClassVar[int]
    NUM_RETURN_SEQUENCES_FIELD_NUMBER: _ClassVar[int]
    REPETITION_PENALTY_FIELD_NUMBER: _ClassVar[int]
    TEMPERATURE_FIELD_NUMBER: _ClassVar[int]
    TOKEN_WINDOW_FIELD_NUMBER: _ClassVar[int]
    TOP_K_FIELD_NUMBER: _ClassVar[int]
    TOP_P_FIELD_NUMBER: _ClassVar[int]
    do_sample: bool
    early_stopping: bool
    input: str
    length_penalty: float
    max_length: int
    min_length: int
    model: str
    no_repeat_ngram_size: int
    num_beam_groups: int
    num_beams: int
    num_return_sequences: int
    repetition_penalty: float
    temperature: float
    token_window: int
    top_k: int
    top_p: float
    def __init__(self, model: _Optional[str] = ..., input: _Optional[str] = ..., token_window: _Optional[int] = ..., min_length: _Optional[int] = ..., max_length: _Optional[int] = ..., do_sample: bool = ..., early_stopping: bool = ..., top_p: _Optional[float] = ..., top_k: _Optional[int] = ..., temperature: _Optional[float] = ..., repetition_penalty: _Optional[float] = ..., length_penalty: _Optional[float] = ..., num_beams: _Optional[int] = ..., num_beam_groups: _Optional[int] = ..., num_return_sequences: _Optional[int] = ..., no_repeat_ngram_size: _Optional[int] = ...) -> None: ...
