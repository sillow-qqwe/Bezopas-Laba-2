package main

import (
	"fmt"
	"math"
	"os"
)

// Глобальная переменная с неиспользуемым импортом (для демонстрации)
var unusedGlobal int = 100 // Не используется в программе

type Config struct {
	Name string
	Age  int
}

// Функция с типичными ошибками
func processData(numbers []int) (int, error) {
	if numbers == nil || len(numbers) == 0 {
		return 0, fmt.Errorf("empty slice provided") // Ошибка: неиспользуемая сигнатура
	}
	
	sum := 0
	for i := 0; i <= len(numbers); i++ { // Ошибка: выход за границы массива (off-by-one)
		sum += numbers[i]
	}
	
	// Ошибка: неиспользуемая переменная
	unusedVar := "this will be flagged"
	
	// Ошибка: потеря точности при конвертации
	var bigNumber int64 = 9999999999
	smallNumber := int16(bigNumber) // Усечение значения
	
	_ = smallNumber
	_ = unusedVar
	return sum, nil
}

// Демонстрация путаницы с range (возвращает индекс, а не значение)
func findMax(values []int) int {
	maxVal := 0
	for _, val := range values { // Правильно
		if val > maxVal {
			maxVal = val
		}
	}
	// Ошибка: в другом месте перепутан индекс и значение
	for idx := range values {
		fmt.Println("Processing index:", idx) // Замаскированная ошибка
	}
	return maxVal
}

// Ошибка: неиспользуемый параметр
func unusedParam(x int, y string) bool {
	return x > 10 // y не используется
}

func main() {
	// Ошибка: непроверенная ошибка
	file, _ := os.Open("nonexistent.txt")
	defer file.Close() // Паника, если file == nil
	
	// Ошибка: теневое копирование переменной
	err := 1
	if true {
		err := "some error" // Переопределение err
		fmt.Println(err)
	}
	_ = err
	
	// Ошибка: ненужное приведение типов
	val := 42
	check := bool(val != 0) // Избыточное преобразование
	
	// Ошибка: неоправданно сложное условие
	if check == true { // Можно просто if check
		fmt.Println("True")
	}
	
	// Использование math.Pi без необходимости
	fmt.Println(math.Pi)
}
