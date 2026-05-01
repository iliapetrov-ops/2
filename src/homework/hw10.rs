<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Перевірка числа на паліндром</title>
</head>
<body>
<pre id="result" style="font-family: monospace; font-size: 160px; background: #f4f4f4; padding: 100px;"></pre>
<script>
  function isPalindrome(x) {
    if (x < 0) return false;
    let original = x;
    let reversed = 0;
    while (x > 0) {
      reversed = reversed * 10 + (x % 10);
      x = Math.floor(x / 10);
    }
    return original === reversed;
  }
  const tests = [
    { num: 123, expected: false },
    { num: 121, expected: true },
    { num: 1221, expected: true }
  ];

  let allPassed = true;
  let outputLines = [];

  tests.forEach((test, idx) => {
    const result = isPalindrome(test.num);
    const passed = result === test.expected;
    outputLines.push(`Тест ${idx + 1}: isPalindrome(${test.num}) → ${result} | Очікувано: ${test.expected} → ${passed ? "✅" : "❌"}`);
    if (!passed) allPassed = false;
  });
  const extraTests = [
    { num: 0, expected: true },
    { num: 1, expected: true },
    { num: 10, expected: false },
    { num: 12321, expected: true }
  ];

  outputLines.push("\n--- Додаткові тести ---");
  extraTests.forEach((test, idx) => {
    const result = isPalindrome(test.num);
    const passed = result === test.expected;
    outputLines.push(`Тест: isPalindrome(${test.num}) → ${result} | Очікувано: ${test.expected} → ${passed ? "✅" : "❌"}`);
    if (!passed) allPassed = false;
  });

  const summary = allPassed ? "✅ Всі тести пройдено успішно!" : "❌ Деякі тести не пройшли!";
  outputLines.push("\n" + summary);
  const output = outputLines.join("\n");
  console.log(output);
  document.getElementById("result").textContent = output;
</script>
</body>
</html>
