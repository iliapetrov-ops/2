<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Код Ґрея (Gray code)</title>
  <style>
    pre {
      font-family: monospace;
      font-size: 140px;
      background: #f4f4f4;
      padding: 100px;
      margin: 100px;
    }
    button {
      margin: 100px;
      padding: 80px 160px;
      font-size: 160px;
      cursor: pointer;
    }
  </style>
</head>
<body>
<pre id="output"></pre>
<button onclick="runTests()">Запустити тести</button>

<script>
  function gray(n) {
    if (n === 0) return [""];
    const prev = gray(n - 1);
    const result = [];
    for (let code of prev) {
      result.push("0" + code);
    }
    for (let i = prev.length - 1; i >= 0; i--) {
      result.push("1" + prev[i]);
    }
    return result;
  }
  const testData = [
    { n: 0, expected: [""] },
    { n: 1, expected: ["0", "1"] },
    { n: 2, expected: ["00", "01", "11", "10"] }, // Увага: класичний Gray code має порядок 00,01,11,10
    { n: 3, expected: ["000","001","011","010","110","111","101","100"] },
    { n: 4, expected: [
        "0000","0001","0011","0010","0110","0111","0101","0100",
        "1100","1101","1111","1110","1010","1011","1001","1000"
      ] }
  ];
  function arraysEqual(a, b) {
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) return false;
    }
    return true;
  }
  function runTests() {
    let allPassed = true;
    let outputLines = [];

    testData.forEach((test, idx) => {
      const result = gray(test.n);
      const passed = arraysEqual(result, test.expected);
      allPassed = allPassed && passed;

      outputLines.push(`Тест ${idx + 1}: gray(${test.n})`);
      outputLines.push(`Отримано: [${result.join(", ")}]`);
      outputLines.push(`Очікується: [${test.expected.join(", ")}]`);
      outputLines.push(`Результат: ${passed ? "✅" : "❌"}\n`);
    });

    outputLines.push(`=== ВСІ ТЕСТИ ${allPassed ? "ПРОЙДЕНО" : "НЕ ПРОЙДЕНО"} ===`);
    const output = outputLines.join("\n");
    console.log(output);
    document.getElementById("output").textContent = output;
  }
  runTests();
</script>
</body>
</html>
