package utils

func Map[A, B any](slice []A, f func(a A) B) []B {
	out := make([]B, len(slice))
	for i, input := range slice {
		out[i] = f(input)
	}
	return out
}

func Contains[A comparable](slice []A, a A) bool {
	for _, input := range slice {
		if input == a {
			return true
		}
	}
	return false
}

func MapAndContains[A any, B comparable](slice []A, f func(a A) B, value B) bool {
	for _, input := range slice {
		if f(input) == value {
			return true
		}
	}
	return false
}

func Filter[A any](slice []A, f func(a A) bool) []A {
	out := make([]A, 0, len(slice))
	for _, input := range slice {
		if f(input) {
			out = append(out, input)
		}
	}
	return out
}

func Reduce[A, B any](slice []A, f func(a A, b B) B, initial B) B {
	out := initial
	for _, input := range slice {
		out = f(input, out)
	}
	return out
}

func Any[A any](slice []A, predicate func(a A) bool) bool {
	for _, input := range slice {
		if predicate(input) {
			return true
		}
	}
	return false
}

func All[A any](slice []A, predicate func(a A) bool) bool {
	return Reduce(slice, func(a A, b bool) bool {
		return b && predicate(a)
	}, true)
}

func Ternary[A any](condition bool, a, b A) A {
	if condition {
		return a
	}
	return b
}

func Count[A comparable](slice []A, a A) int {
	count := 0
	for _, input := range slice {
		if input == a {
			count++
		}
	}
	return count
}
