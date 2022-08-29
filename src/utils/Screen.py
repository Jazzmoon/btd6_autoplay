import sys
import pytesseract as tesser
from PIL import Image
from typing import Union
global use_grab

try:
    from PIL import ImageGrab, ImageOps, ImageEnhance
    use_grab: bool = True
except Exception as e:
    if sys.platform == "linux":
        from Xlib import display, X
        use_grab: bool = False
    else:
        print(f"PIL unavailable for your system.")
        raise e


class Screen:
    def __init__(self):
        tesser.pytesseract.tesseract_cmd = r'C:\Program Files (x86)\Tesseract-OCR\tesseract'

    @staticmethod
    def screen_grab(rectangle: list[Union[float, int]], color_filter: str = 'white'):
        global use_grab
        x, y, width, height = rectangle

        if use_grab:
            image = ImageGrab.grab(bbox=[x, y, x + width, y + height])

            if color_filter == "gold":
                image_data = image.getdata()
                new_image_data = []

                for data in image_data:
                    if data[0] == 255 and data[1] in range(212, 256) and data[2] == 0:
                        new_image_data.append((255, 255, 255))
                    else:
                        new_image_data.append((0, 0, 0))
                image.putdata(new_image_data)

            elif color_filter == "red":
                image_data = image.getdata()
                new_image_data = []

                for data in image_data:
                    if data[0] == 255:
                        new_image_data.append((255, 255, 255))
                    else:
                        new_image_data.append((0, 0, 0))
                image.putdata(new_image_data)

            enhancer = ImageEnhance.Contrast(image)
            image = enhancer.enhance(2)
            image = ImageOps.invert(image)
            image = image.point(lambda x: 0 if x < 10 else 255)
            image = image.convert('L')
        else:
            dsp = display.Display()
            root = dsp.screen().root
            raw_image = root.get_image(x, y, width, height, X.ZPixmap, 0xffffffff)
            image = Image.frombuffer(
                "RGB", (width, height), raw_image.data, "raw", "BGRX", 0, 1)

        return image

