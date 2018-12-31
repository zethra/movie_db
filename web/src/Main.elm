import Browser
import Browser.Dom
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)

main =
    Browser.sandbox { init = init, update = update, view = view }

type alias Model = Int

init: Model
init = 0


type Msg = I | D


update : Msg -> Model -> Model
update msg model =
    case msg of
        I ->
            model + 1

        D ->
            model - 1


view : Model -> Html Msg
view model =
    div []
        [ button [ onClick D ] [text "-" ]
        , div [] [text (String.fromInt model)]
        , button [ onClick I] [text "+"]
        ]