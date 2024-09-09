module [
    AccessKey,
    AccessKeyId,
    AccessKeyAdapters,
    CreateAccessKeyError,
    createAccessKey,
]

AccessKeyId : Str

AccessKey : {
    id : AccessKeyId,
    key : Str,
}

CreateAccessKeyError : [AccessKeyAlreadyExists AccessKeyId]

PersistAccessKeyError : [DuplicateId AccessKeyId]
AccessKeyAdapters : {
    persistNewAccessKey : AccessKey -> Task {} PersistAccessKeyError,
}

createAccessKey : AccessKeyAdapters -> Task {} CreateAccessKeyError
createAccessKey = \{ persistNewAccessKey } ->
    id = generateId
    key = generateKey
    client = { id, key }

    Task.mapErr (persistNewAccessKey (client)) \err ->
        when err is
            DuplicateId dup -> AccessKeyAlreadyExists dup

generateId =
    "TODO"

generateKey =
    "TODO"

expect
    # When invoked, persists an access key and returns it
    stubAdapters = {
        persistNewAccessKey: \_ -> Task.ok {},
    }

    key = stubAdapters |> createAccessKey

    # TODO expectation on Tasks?
    Bool.false
