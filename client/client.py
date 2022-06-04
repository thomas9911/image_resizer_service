import grpc
import resizer_pb2_grpc
import resizer_pb2

import PySimpleGUI as sg
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
import json
import base64
import os

SHARED_SECRET = b"kqB7xA9RUVsuWlud89+wLJGqznMZIbKqTznEzn0N7u4="
ACCESS_KEY = "test_key_id"
SECRET_KEY = "secret_access_key"


def pixel_valid_maker(window, values, event, key):
    if event == key and values[key]:
        try:
            if 0 < int(values[key]) < 8000:
                pass
            else:
                raise ValueError
        except:
            window[key].update(values[key][:-1])


def make_config():
    return {
        "region": "us-east-1",
        "endpoint": "http://localhost:9000",
        "access_key": ACCESS_KEY,
        "secret_access_key": SECRET_KEY,
        "nonce": os.urandom(16).hex(),
    }


def encode_config(config):
    secret_bytes = base64.b64decode(SHARED_SECRET)
    data = json.dumps(config)
    encoder = AESGCM(secret_bytes)
    iv = os.urandom(12)
    return iv + encoder.encrypt(iv, data.encode(), None)


def gui(stub):
    sg.theme("DarkAmber")
    layout = [
        [sg.Text("Resizing")],
        [sg.Text("Bucket"), sg.InputText("hallo", key="-BUCKET-")],
        [sg.Text("Input key"), sg.InputText(key="-INPUT-")],
        [sg.Text("Output key"), sg.InputText(key="-OUTPUT-")],
        [sg.Text("Width"), sg.InputText("100", key="-WIDTH-", enable_events=True)],
        [sg.Text("Height"), sg.InputText("100", key="-HEIGHT-", enable_events=True)],
        [sg.Output(size=(80, 20))],
        [sg.Button("Resize"), sg.Button("Cancel")],
    ]

    window = sg.Window("Resizing", layout)

    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED or event == "Cancel":
            break
        pixel_valid_maker(window, values, event, "-WIDTH-")
        pixel_valid_maker(window, values, event, "-HEIGHT-")

        if event == "Resize":
            response = stub.Resize(
                resizer_pb2.ResizeRequest(
                    input=values["-INPUT-"],
                    output=values["-OUTPUT-"],
                    bucket=values["-BUCKET-"],
                    width=int(values["-WIDTH-"]),
                    height=int(values["-HEIGHT-"]),
                    config=encode_config(make_config()),
                )
            )
            print("Client received: " + response.message)

    window.close()


def run():
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = resizer_pb2_grpc.ResizerStub(channel)
        gui(stub)


if __name__ == "__main__":
    run()
