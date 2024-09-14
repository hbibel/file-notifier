module [
    AccessKey,
    AccessKeyId,
    AccessKeyAdapters,
    CreateAccessKeyError,
    createAccessKey,
    generateId, # TODO unexport
]
import Lib.RandomUtils

AccessKeyId : Str

AccessKey : {
    id : AccessKeyId,
    key : Str,
}

CreateAccessKeyError : [AccessKeyAlreadyExists AccessKeyId, OtherError Str]

PersistAccessKeyError : [DuplicateId AccessKeyId]
AccessKeyAdapters : {
    persistNewAccessKey : AccessKey -> Task {} PersistAccessKeyError,
}

createAccessKey : AccessKeyAdapters -> Task {} CreateAccessKeyError
createAccessKey = \{ persistNewAccessKey } ->
    id = generateId!
    key = generateKey
    client = { id, key }

    Task.mapErr (persistNewAccessKey (client)) \err ->
        when err is
            DuplicateId dup -> AccessKeyAlreadyExists dup

accessKeyChars =
    List.range { start: At 30, end: At 39 } # digits
    |> List.concat (List.range { start: At 65, end: At 90 }) # uppercase chars
    |> List.concat (List.range { start: At 97, end: At 122 }) # lowercase chars

generateId : Task Str CreateAccessKeyError
generateId =
    Lib.RandomUtils.generateString accessKeyChars 12
    |> Task.mapErr \_ -> OtherError "Failed to generate a random string"

generateKey : Str
generateKey =
    "123"
