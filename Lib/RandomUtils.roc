module [
    generateNumber,
    generateString,
]
import pf.Utc
import rand.Random

generateNumber : I32, I32 -> Task I32 *
generateNumber = \from, to ->
    seed = Task.map Utc.now \now ->
        asNanos = Utc.toNanosSinceEpoch now
        Num.toU32 (asNanos % Num.toI128 Num.maxU32) |> Random.seed
    gen = Random.int from to
    Task.map seed \s ->
        (gen s).value

generateString : List U8, U64 -> Task Str [RandomGenerationError _]
generateString = \chars, len ->
    initialSeed = (Task.mapErr timebasedSeed RandomGenerationError)!

    indexGen = Random.int 0 (Num.toI32 (List.len chars))

    accF = \state, _ ->
        when state is
            Ok { seed, accChars } ->
                randState = indexGen seed
                randomIndex = Num.toU64 randState.value
                List.get chars randomIndex
                |> Result.map \randomChar ->
                    { seed: randState.state, accChars: List.append accChars randomChar }

            error ->
                error

    initialAcc = Ok { seed: initialSeed, accChars: [] }

    lenTimes = List.repeat 0 len
    List.walk lenTimes initialAcc accF
    |> Result.map .accChars
    |> Result.try Str.fromUtf8
    |> Result.mapErr RandomGenerationError
    |> Task.fromResult

timebasedSeed : Task (Random.State U32) _
timebasedSeed =
    Task.map Utc.now \now ->
        asNanos = Utc.toNanosSinceEpoch now
        Num.toU32 (asNanos % Num.toI128 Num.maxU32) |> Random.seed
