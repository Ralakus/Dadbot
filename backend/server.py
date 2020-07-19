import pyruvate
import api

print("Starting server")
pyruvate.serve(api.application, "0.0.0.0:80", 1)
