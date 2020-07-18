import falcon
import bot

class DadbotAPI:
    def on_post(self, req, resp):
        body = req.stream.read()
        resp.body = str(bot.chatbot.get_response(str(body)))
    def on_get(self, req, resp):
        resp.body = "Please post a request"

application = api = falcon.API()
api.add_route('/poll', DadbotAPI())