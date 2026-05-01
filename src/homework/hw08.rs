<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Перевірка числа на простоту</title>
</head>
<body>
<pre id="primeResult" style="font-family: monospace; font-size: 400px; background: #f4f4f4; padding: 300px;"></pre>
<script>
  const NUMBER = 29;
  function isPrime(n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 === 0 || n % 3 === 0) return false;
    for (let i = 5; i * i <= n; i += 6) {
      if (n % i === 0 || n % (i + 2) === 0) return false;
    }
    return true;
  }

  const result = isPrime(NUMBER);
  const output = `Число: ${NUMBER}\nПросте? ${result ? "Так" : "Ні"}`;
  console.log(output);
  document.getElementById("primeResult").textContent = output;
</script>
</body>
</html>
