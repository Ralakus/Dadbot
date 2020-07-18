
from chatterbot import ChatBot
from chatterbot.trainers import ChatterBotCorpusTrainer
from chatterbot.trainers import ListTrainer
from chatterbot.trainers import UbuntuCorpusTrainer

chatbot = ChatBot("Daddy")

from os import path

if not path.exists("sentence_tokenizer.pickle"):
    corpustrainer = ChatterBotCorpusTrainer(chatbot)
    corpustrainer.train("chatterbot.corpus.english")