package utils

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"image/jpeg"

	"github.com/go-vgo/robotgo"

	"github.com/anthonynsimon/bild/adjust"
	"github.com/anthonynsimon/bild/effect"
	"github.com/anthonynsimon/bild/paint"
	"github.com/anthonynsimon/bild/segment"
	"github.com/anthonynsimon/bild/transform"
)

type Threshold uint8

const (
	DarkBackground  Threshold = 128
	LightBackground Threshold = 1
)

func ProcessImage(img image.Image, threshold Threshold) ([]byte, error) {
	// get bounds of the image
	bounds := img.Bounds()

	img = transform.Resize(img, bounds.Dx()*4, bounds.Dy()*4, transform.NearestNeighbor)

	img = adjust.Contrast(img, 0.5)
	img = segment.Threshold(img, uint8(threshold))
	img = effect.Grayscale(img)
	img = paint.FloodFill(img, image.Point{X: 0, Y: 0}, color.Black, 150)
	img = transform.ShearV(img, 4)
	img = effect.Invert(img)

	var buff bytes.Buffer
	if err := jpeg.Encode(&buff, img, &jpeg.Options{Quality: 75}); err != nil {
		return nil, err
	}

	return buff.Bytes(), nil

}

// Capture the screen with JPEG encoding as a byte array
func CaptureScreenAsJpeg(threshold Threshold, text string, args ...int) ([]byte, error) {
	sshot := robotgo.CaptureImg(args...)
	if IsDebug() {
		robotgo.SaveJpeg(sshot, fmt.Sprintf("debug/screenshot-%s.jpg", text), 100)
	}

	screenShot, err := ProcessImage(sshot, threshold)
	if err != nil {
		return nil, err
	}

	if IsDebug() {
		image, _, err := image.Decode(bytes.NewReader(screenShot))
		if err != nil {
			return nil, err
		}
		robotgo.SaveJpeg(image, fmt.Sprintf("debug/screenshot-bw-%s.jpg", text), 100)
	}

	return screenShot, nil
}
