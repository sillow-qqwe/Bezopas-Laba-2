import os
import pickle
import subprocess
import hashlib
import sqlite3

# Глобальные переменные с секретами (Bandit: B105)
PASSWORD = "SuperSecretPassword123"
API_TOKEN = "sk-1234567890abcdef"

# Использование assert для проверки прав (Bandit: B101)
def delete_user_file(username):
    assert username == "admin", "Access denied"
    os.remove(f"/var/data/{username}.txt")

# Использование pickle с ненадёжными данными (Bandit: B301)
def load_session(session_file):
    data = open(session_file, 'rb').read()
    return pickle.loads(data)

# Прямое выполнение системных команд с пользовательским вводом (Bandit: B602, B603)
def ping_host(host):
    # Пользователь может ввести "; rm -rf /"
    command = f"ping -c 4 {host}"
    process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE)
    return process.stdout.read()

# Слабый алгоритм хеширования (Bandit: B303)
def hash_password_md5(plain_text):
    return hashlib.md5(plain_text.encode()).hexdigest()

# Потенциальная SQL-инъекция (Bandit: B608)
def get_user_info(username):
    conn = sqlite3.connect("users.db")
    cursor = conn.cursor()
    # Форматирование строки вместо параметризованного запроса
    query = f"SELECT * FROM users WHERE username = '{username}'"
    cursor.execute(query)
    return cursor.fetchone()

# Использование временного файла с предсказуемым именем (Bandit: B108)
def write_temp_log(data):
    temp_path = "/tmp/app_debug.log"
    with open(temp_path, "w") as f:
        f.write(data)

# Уязвимость при работе с YAML (Bandit: B506) - имитация
def parse_config(path):
    # eval от пользовательского ввода (Bandit: B307)
    with open(path) as f:
        content = f.read()
        # Очень плохая идея
        config = eval(content)
    return config

# Запись хардкоденных ключей в файл (Bandit: B106)
def setup_environment():
    with open(".env", "w") as f:
        f.write(f"SECRET_KEY={PASSWORD}\n")
        f.write(f"DEBUG=True\n")
    # Установка опасных прав (Bandit: B103)
    os.chmod(".env", 0o777)

# Основная функция
if __name__ == "__main__":
    user_input = input("Enter username: ")
    print(get_user_info(user_input))
    
    host_input = input("Ping host: ")
    ping_host(host_input)
    
    setup_environment()
    
    # Вызов опасной функции eval напрямую
    result = eval("2 + 2")  # B307
    print(f"Eval result: {result}")
