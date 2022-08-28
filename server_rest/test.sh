#! /bin/bash

# curl --data @demo.jpg -o out.jpg http://localhost:3030/resize

# curl -v -F filename=demo.jpg -F upload=@demo.jpg -o out.jpg 'http://localhost:3030/resize_multipart?x=150&y=150'

# curl -F filename=demo.jpg -F upload=@demo.jpg 'http://localhost:3030/resize_multipart?x=150&y=150'


curl -v -H "content-type: image/jpeg" --data-binary  @demo.jpg -o out.jpg 'http://localhost:3030/resize_binary?x=150&y=150'
