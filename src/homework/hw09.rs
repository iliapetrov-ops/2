<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Зсув рядка (rotate)</title>
</head>
<body>
<pre id="output" style="font-family: monospace; font-size: 30px; background: #f4f4f4; padding: 15px;"></pre>
<script>
  function rotate(s, n) {
    const len = s.length;
    if (len === 0) return s;
    let shift = ((n % len) + len) % len;
    if (shift === 0) return s;
    return s.slice(-shift) + s.slice(0, len - shift);
  }
  const s = "abcdefgh";
  const tests = [
    { n: 0,  expected: "abcdefgh" },
    { n: 8,  expected: "abcdefgh" },
    { n: -8, expected: "abcdefgh" },
    { n: 1,  expected: "habcdefg" },
    { n: 2,  expected: "ghabcdef" },
    { n: 10, expected: "ghabcdef" },
    { n: -1, expected: "bcdefgha" },
    { n: -2, expected: "cdefghab" },
    { n: -10, expected: "cdefghab" }
  ];

  let allPassed = true;
  let results = [];

  tests.forEach((test, idx) => {
    const result = rotate(s, test.n);
    const passed = result === test.expected;
    results.push(`Тест ${idx + 1}: rotate("${s}", ${test.n}) → "${result}" | Очікувано: "${test.expected}" → ${passed ? "✅" : "❌"}`);
    if (!passed) allPassed = false;
  });

  const summary = allPassed ? "✅ Всі тести пройдено успішно!" : "❌ Деякі тести не пройшли!";
  const output = results.join("\n") + "\n\n" + summary;
  console.log(output);
  document.getElementById("output").textContent = output;
</script>
</body>
</html>
