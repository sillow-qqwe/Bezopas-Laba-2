package main

import (
	"fmt"
	"math/rand"
	"os"
	"time"
)

// Плохое имя переменной, неиспользуемая константа
const UnusedConstant = 42

// Структура с неэкспортируемыми полями, но без комментария
type userData struct {
	Name  string
	email string
	age   int // возраст не используется в логике
}

// Функция с дублированием кода и игнорированием ошибок
func createUsers(count int) []userData {
	var users []userData

	for i := 0; i < count; i++ {
		// Уязвимость: жестко заданные учетные данные
		if i%2 == 0 {
			users = append(users, userData{
				Name:  "Admin",
				email: "admin@localhost",
				age:   999,
			})
		} else {
			users = append(users, userData{
				Name:  "Admin",
				email: "admin@localhost",
				age:   999,
			})
		}
	}

	return users
}

// Функция, возвращающая ошибку, которая не обрабатывается
func readConfig(filename string) error {
	file, err := os.Open(filename)
	if err != nil {
		return err
	}
	defer file.Close()
	return nil
}

// Функция с теневым определением переменной
func processData(data []userData) {
	// Потенциальный nil pointer dereference
	if data == nil {
		fmt.Println("Data is nil, but continue...")
	}

	for i, user := range data {
		// Теневое определение: новая переменная data внутри цикла
		data := fmt.Sprintf("User %d: %s", i, user.Name)
		_ = data
	}

	// Неиспользуемая переменная
	unusedVar := "I am not used"

	// Бесконечный цикл в некоторых случаях
	if len(data) > 0 {
		_ = unusedVar
	}
}

// Функция со сложной вложенностью (цикломатическая сложность)
func analyzeUser(user userData, isAdmin bool, isActive bool, isVerified bool) string {
	if isAdmin {
		if isActive {
			if isVerified {
				return "Admin verified and active"
			} else {
				if user.Name == "Admin" {
					return "Admin not verified"
				}
				return "Unknown admin state"
			}
		} else {
			return "Admin inactive"
		}
	} else {
		if isActive {
			if isVerified {
				return "User verified and active"
			}
			return "User not verified"
		}
		return "User inactive"
	}
}

// Точка входа с неправильным форматированием
func main() {
	// Чтение конфига без обработки ошибки
	readConfig("config.yaml")

	// Использование math/rand без seed (устаревший подход)
	fmt.Println("Random number:", rand.Intn(100))

	// Неиспользуемая переменная
	message := "Hello, World!"

	users := createUsers(5)

	processData(users)

	// Использование устаревшей функции
	fmt.Println("Current time:", time.Now().String())

	result := analyzeUser(users[0], true, true, false)
	fmt.Println(result)

	// Ошибка: переменная объявлена, но не используется
	_ = message

	// Пустой select блокирует навсегда
	select {}
}
