import grpc
import helloworld_pb2_grpc
import helloworld_pb2

import PySimpleGUI as sg


def pixel_valid_maker(window, values, event, key):
    if event == key and values[key]:
        try:
            if 0 < int(values[key]) < 4000:
                pass
            else:
                raise ArgumentError
        except:
            window[key].update(values[key][:-1])


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
                helloworld_pb2.ResizeRequest(
                    input=values["-INPUT-"],
                    output=values["-OUTPUT-"],
                    bucket=values["-BUCKET-"],
                    width=int(values["-WIDTH-"]),
                    height=int(values["-HEIGHT-"]),
                )
            )
            print("Client received: " + response.message)

    window.close()


def run():
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = helloworld_pb2_grpc.ResizerStub(channel)
        # response = stub.Resize(
        #     helloworld_pb2.ResizeRequest(
        #         input="hello.png",
        #         output="out.png",
        #         bucket="hallo",
        #         width=120,
        #         height=120,
        #     )
        # )
        # print("Greeter client received: " + response.message)
        gui(stub)


if __name__ == "__main__":
    run()
    # gui()
