package main

import (
	"fmt"
	"math"
	"os"
)

// глобальная переменная, которая нигде не используется
var unusedGlobal int = 100

type Config struct {
	Name string
	Age  int
}

func processData(numbers []int) (int, error) {
	if numbers == nil || len(numbers) == 0 {
		return 0, fmt.Errorf("empty slice provided") // неиспользуемая сигнатура
	}
	
	sum := 0
	for i := 0; i <= len(numbers); i++ { // выход за границы массива (off-by-one)
		sum += numbers[i]
	}
	
	// неиспользуемая переменная
	unusedVar := "this will be flagged"
	
	// потеря точности при конвертации, тк усекается значение
	var bigNumber int64 = 9999999999
	smallNumber := int16(bigNumber)
	
	_ = smallNumber
	_ = unusedVar
	return sum, nil
}

// range (возвращает индекс, а не значение)
func findMax(values []int) int {
	maxVal := 0
	for _, val := range values {
		if val > maxVal {
			maxVal = val
		}
	}
	// в другом месте перепутан индекс и значение
	for idx := range values {
		fmt.Println("Processing index:", idx)
	}
	return maxVal
}

// неиспользуемый параметр
func unusedParam(x int, y string) bool {
	return x > 10
}

func main() {
	file, _ := os.Open("nonexistent.txt")
	defer file.Close() // Паника, если file == nil
	
	// теневое копирование переменной
	err := 1
	if true {
		err := "some error" // переопределение err
		fmt.Println(err)
	}
	_ = err
	
	// ненужное приведение типов
	val := 42
	check := bool(val != 0)
	
	// неоправданно сложное условие
	if check == true { // Можно просто if check
		fmt.Println("True")
	}
	
	// использование math.Pi без необходимости
	fmt.Println(math.Pi)
}
