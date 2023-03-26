import img_and_base64

result = img_and_base64.img_to_base64(
    input("Enter image's path:\n"),
    input(
        "If you wanna convert some text string to base64, type it here (blank line should be converted too):\n"
    ),
)
print(result)
