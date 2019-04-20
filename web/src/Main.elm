module Main exposing (Model, Msg(..), init, main, update, view)

import Browser
import Browser.Dom
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- Model


type alias Movie =
    { id : Int
    , title : String
    , rating : String
    , category : String
    , format : String
    , aspect : String
    , actors : String
    , drawer : String
    , column : String
    }


type alias Model =
    Int


init : () -> ( Model, Cmd Msg )
init _ =
    ( 0, Cmd.none )



-- Update


type Msg
    = NOP


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    ( model, Cmd.none )



-- Subscriptions


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- View


view : Model -> Html Msg
view model =
    div []
        [ header []
            [ h1 [] [ text "Movie DB" ]
            ]
        ]
