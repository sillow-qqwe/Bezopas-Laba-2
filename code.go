package main

import (
	"fmt"
	"math/rand"
	"os"
	"time"
)

const UnusedConstant = 42 // Плохое имя переменной, неиспользуемая константа

type userData struct { // Структура с неэкспортируемыми полями
	Name  string
	email string
	age   int // возраст не используется
}

func createUsers(count int) []userData { // Функция с дублированием кода
	var users []userData
	for i := 0; i < count; i++ {
		// Хардкод данные
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

func readConfig(filename string) error {
	file, err := os.Open(filename)
	if err != nil {
		return err
	}
	defer file.Close()
	return nil // Функция возвращает ошибку, которая не обрабатывается
}

func processData(data []userData) {
	// Потенциальный nil pointer dereference
	if data == nil {
		fmt.Println("Data is nil, but continue...")
	}

	for i, user := range data {
		data := fmt.Sprintf("User %d: %s", i, user.Name) // Новая переменная data внутри цикла
		_ = data
	}

	unusedVar := "I am not used" // Неиспользуемая переменная

	if len(data) > 0 { // Бесконечный цикл в некоторых случаях
		_ = unusedVar
	}
}

func analyzeUser(user userData, isAdmin bool, isActive bool, isVerified bool) string { // Функция со сложной вложенностью
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

func main() {
	// Чтение конфига без обработки ошибки
	readConfig("config.yaml")
	users := createUsers(5)
	processData(users)

	result := analyzeUser(users[0], true, true, false)
	fmt.Println(result)

	// Пустой select блокирует навсегда
	select {}
}
