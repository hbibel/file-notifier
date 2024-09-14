app [Model, server] {
    pf: platform "https://github.com/roc-lang/basic-webserver/releases/download/0.9.0/taU2jQuBf-wB8EJb0hAkrYLYOGacUU5Y9reiHG45IY4.tar.br",
    rand: "https://github.com/lukewilliamboswell/roc-random/releases/download/0.2.1/mJSD8-uN-biRqa6CiqdN4-VJsKXxY8b1eFf6mFTe93A.tar.br",
}

import pf.Stdout
import pf.Http exposing [Request, Response]
import pf.Utc
import Application.AccessKeys

# Model is produced by `init`.
Model : {}

# With `init` you can set up a database connection once at server startup,
# generate css by running `tailwindcss`,...
# In this case we don't have anything to initialize, so it is just `Task.ok {}`.

server = { init: Task.ok {}, respond }

respond : Request, Model -> Task Response [ServerErr Str]_
respond = \req, _ ->
    # Log request datetime, method and url
    datetime = Utc.now! |> Utc.toIso8601Str

    id = Application.AccessKeys.generateId!
    dbg id

    # name : Clients.AccessKey
    # name = "something"
    # dbg name

    Stdout.line! "$(datetime) $(Http.methodToStr req.method) $(req.url)"

    Task.ok { status: 200, headers: [], body: Str.toUtf8 "<b>Hello, web!</b></br>" }
