from PIL import Image

image = Image.open(input("Filename: "))
level = ""

for y in range(0, image.height):
    row = ""
    for x in range(0, image.width):
        color = image.getpixel((x, y)) 
        weight = color[3]
        if weight > 160:
            row += "1"
        else:
            row += "0"
    level += row
    level += "\n"

f = open(input("Output: "), "w")
f.write(level)