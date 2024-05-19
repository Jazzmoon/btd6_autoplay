package utils

import (
	"bytes"
	"image"
	"image/jpeg"

	"github.com/go-vgo/robotgo"

	"github.com/anthonynsimon/bild/adjust"
	"github.com/anthonynsimon/bild/effect"
	"github.com/anthonynsimon/bild/transform"
)

func ProcessImage(image image.Image) ([]byte, error) {
	// get bounds of the image
	bounds := image.Bounds()

	img := transform.Resize(image, bounds.Dx()*4, bounds.Dy()*4, transform.NearestNeighbor)
	//img = effect.Grayscale(img)
	img = effect.GrayscaleWithWeights(img, 0.25, 0.2, 0.3)
	img = adjust.Contrast(img, 0.9)
	//img = adjust.Brightness(img, 0.1)
	img = effect.Invert(img)
	//img = effect.Sobel(img)

	var buff bytes.Buffer
	if err := jpeg.Encode(&buff, img, &jpeg.Options{Quality: 75}); err != nil {
		return nil, err
	}

	return buff.Bytes(), nil

}

// Capture the screen with JPEG encoding as a byte array
func CaptureScreenAsJpeg(args ...int) ([]byte, error) {
	sshot := robotgo.CaptureImg(args...)
	if isDebug() {
		robotgo.SaveJpeg(sshot, "screenshot.jpg", 100)
	}

	screenShot, err := ProcessImage(sshot)
	if err != nil {
		return nil, err
	}

	if isDebug() {
		image, _, err := image.Decode(bytes.NewReader(screenShot))
		if err != nil {
			return nil, err
		}
		robotgo.SaveJpeg(image, "screenshot-bw.jpg", 100)
	}

	return screenShot, nil
}
