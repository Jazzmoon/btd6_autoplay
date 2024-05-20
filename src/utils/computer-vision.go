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

func ProcessImage(orignalImg image.Image) ([]byte, error) {
	// get bounds of the image
	bounds := orignalImg.Bounds()

	img := transform.Resize(orignalImg, bounds.Dx()*5, bounds.Dy()*5, transform.NearestNeighbor)
	//img = paint.FloodFill(img, image.Point{50, 50}, color.RGBA{255, 0, 0, 255}, 255)
	//img = effect.Grayscale(img)
	img = effect.Invert(img)
	img = effect.GrayscaleWithWeights(img, 0.45, 0.03, 0.03)
	img = adjust.Contrast(img, 0.9)
	img = adjust.Brightness(img, 0.3)

	//img = effect.Sobel(img)

	result := image.NewGray(img.Bounds())
	for y := img.Bounds().Min.Y; y < img.Bounds().Max.Y; y++ {
		for x := img.Bounds().Min.X; x < img.Bounds().Max.X; x++ {
			result.Set(x, y, img.At(x, y))
		}
	}

	// Only keep black pixels
	//result = segment.Threshold(result, 100)

	var buff bytes.Buffer
	if err := jpeg.Encode(&buff, result, &jpeg.Options{Quality: 75}); err != nil {
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
