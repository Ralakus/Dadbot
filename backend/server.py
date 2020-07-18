import pyruvate
import api

print("Starting server")
pyruvate.serve(api.application, "127.0.0.1:8080", 1)
