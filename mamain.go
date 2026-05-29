package main

import (
	"fmt"
	"math"
	"sync"
	"time"
)

// Глобальная переменная, которая не используется
var defaultTimeout = 5 * time.Second

// Структура для передачи задач между горутинами
type DataChunk struct {
	ID    int
	Items []int
}

// Результат обработки
type ProcessResult struct {
	ChunkID int
	Sum     int
	Max     int
}

// Функция-воркер с типичными ошибками для линтера
func worker(id int, chunks <-chan DataChunk, results chan<- ProcessResult, wg *sync.WaitGroup) {
	defer wg.Done()
	
	for chunk := range chunks {
		// ОШИБКА: неиспользуемая переменная
		workerName := fmt.Sprintf("worker-%d", id)
		
		// ОШИБКА: выход за границы слайса (off-by-one)
		sum := 0
		for i := 0; i <= len(chunk.Items); i++ {
			sum += chunk.Items[i] // Паника при i == len(chunk.Items)
		}
		
		// ОШИБКА: теневое переопределение переменной
		max := 0
		if len(chunk.Items) > 0 {
			max := chunk.Items[0] // Новая переменная внутри блока
			for _, val := range chunk.Items {
				if val > max {
					max = val
				}
			}
		}
		_ = workerName
		
		// ОШИБКА: усечение данных при конвертации
		var bigValue int64 = 1_000_000_000_000
		smallValue := int32(bigValue) // Потеря данных
		_ = smallValue
		
		// ОШИБКА: игнорирование потенциальной ошибки
		result := ProcessResult{
			ChunkID: chunk.ID,
			Sum:     sum,
			Max:     max,
		}
		
		// Отправка результата с риском блокировки (если буфер мал)
		results <- result
	}
}

// Функция с неиспользуемым параметром
func monitorProgress(total int, done int, verbose bool) string {
	percentage := float64(done) / float64(total) * 100
	// ОШИБКА: параметр verbose не используется
	return fmt.Sprintf("Progress: %.2f%%", percentage)
}

// Функция-диспетчер, запускающая горутины
func processDataset(data []DataChunk, numWorkers int) []ProcessResult {
	// ОШИБКА: проверка условия, которая всегда true
	if numWorkers > 0 || numWorkers < 100 {
		numWorkers = 3 // Мертвая логика
	}
	
	// Каналы для общения с горутинами
	chunks := make(chan DataChunk, len(data))
	results := make(chan ProcessResult, len(data))
	
	// WaitGroup для синхронизации
	var wg sync.WaitGroup
	
	// Запуск воркеров
	for w := 0; w < numWorkers; w++ {
		wg.Add(1)
		go worker(w, chunks, results, &wg) // Запуск горутины
	}
	
	// ОШИБКА: отправка данных в канал без горутины (может заблокироваться)
	for _, chunk := range data {
		chunks <- chunk // Опасно в main-горутине без буфера нужного размера
	}
	close(chunks)
	
	// Ожидание завершения в отдельной горутине
	go func() {
		wg.Wait()
		close(results)
	}()
	
	// ОШИБКА: сбор результатов без проверки закрытия канала
	var finalResults []ProcessResult
	for res := range results {
		finalResults = append(finalResults, res)
	}
	
	// ОШИБКА: неиспользуемый импорт math
	_ = math.Pi
	
	return finalResults
}

// ОШИБКА: функция с сигнатурой, допускающей ошибку, но никогда её не возвращающей
func riskyCalculation(values []int) (int, error) {
	if len(values) == 0 {
		return 0, fmt.Errorf("empty slice")
	}
	
	total := 0
	for _, v := range values {
		total += v
	}
	
	// ОШИБКА: всегда возвращаем nil-ошибку
	return total, nil
}

func main() {
	// Подготовка тестовых данных
	testData := []DataChunk{
		{ID: 1, Items: []int{1, 2, 3, 4, 5}},
		{ID: 2, Items: []int{10, 20, 30}},
		{ID: 3, Items: []int{100, 200}},
		{ID: 4, Items: []int{}},
	}
	
	// ОШИБКА: создание переменной, которая сразу переопределяется
	numWorkers := 10
	numWorkers = 2
	
	// ОШИБКА: игнорируем возвращаемое значение
	_ = monitorProgress(len(testData), 0, true)
	
	// Запуск обработки
	finalResults := processDataset(testData, numWorkers)
	
	// ОШИБКА: неэффективный способ проверки длины
	if len(finalResults) != 0 == true {
		fmt.Println("Processing completed!")
		for _, res := range finalResults {
			fmt.Printf("Chunk %d: Sum=%d, Max=%d\n", res.ChunkID, res.Sum, res.Max)
		}
	}
	
	// ОШИБКА: гонка данных (data race)
	counter := 0
	for i := 0; i < 1000; i++ {
		go func() {
			counter++ // Одновременный доступ без синхронизации
		}()
	}
	
	// Даём время горутинам отработать (плохая практика)
	time.Sleep(100 * time.Millisecond)
	fmt.Printf("Final counter (unsafe): %d\n", counter)
}
