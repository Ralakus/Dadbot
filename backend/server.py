import pyruvate
import api

print("Starting server")
pyruvate.serve(api.application, "127.0.0.1:80", 1)
