package utils

import (
	"reflect"
)

func NonNilFields(v interface{}) []string {
	val := reflect.ValueOf(v)
	typeOfVal := val.Type()

	var fields []string
	for i := 0; i < val.NumField(); i++ {
		if !val.Field(i).IsNil() {
			fields = append(fields, typeOfVal.Field(i).Name)
		}
	}

	return fields
}

func FieldExists(v interface{}, fieldName string) bool {
	r := reflect.ValueOf(v)
	f := r.FieldByName(fieldName)
	return f.IsValid()
}

func GetFieldValue(v interface{}, fieldName string) interface{} {
	r := reflect.ValueOf(v)
	f := r.FieldByName(fieldName)
	return f.Interface()
}
