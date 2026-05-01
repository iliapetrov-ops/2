<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Мінімальна сума сусідніх пар</title>
  <style>
    pre {
      font-family: monospace;
      font-size: 14px;
      background: #f4f4f4;
      padding: 10px;
      margin: 10px;
    }
    button {
      margin: 10px;
      padding: 8px 16px;
      font-size: 16px;
      cursor: pointer;
    }
  </style>
</head>
<body>
<pre id="output"></pre>
<button onclick="runAndDisplay()">Згенерувати новий вектор</button>

<script>
  function gen_random_vector(n) {
    const arr = [];
    for (let i = 0; i < n; i++) {
      arr.push(Math.floor(Math.random() * 90) + 10); // 10..99
    }
    return arr;
  }
  function min_adjacent_sum(data) {
    let minIdx = 0;
    let minSum = data[0] + data[1];
    for (let i = 1; i < data.length - 1; i++) {
      const sum = data[i] + data[i+1];
      if (sum < minSum) {
        minSum = sum;
        minIdx = i;
      }
    }
    return { index: minIdx, sum: minSum };
  }
  function formatIndexesLine(n) {
    let line = "indexes: ";
    for (let i = 0; i < n; i++) {
      line += `${i}.`;
      if (i < n - 1) {
        line += "  ";
      }
    }
    const parts = [];
    for (let i = 0; i < n; i++) {
      parts.push(`${i}.`);
    }
    return "indexes: " + parts.join("  ");
  }
  function formatDataLine(data) {
    return "data:  [" + data.join(", ") + "]";
  }
  function formatUnderlineLine(n, minIdx) {
    const indexesStr = formatIndexesLine(n);
    const prefix = "indexes: ";
    const indexesOnly = indexesStr.slice(prefix.length);
    const lineChars = Array(indexesOnly.length).fill(' ');
    let pos = -1;
    for (let i = 0; i < indexesOnly.length; i++) {
      const str = indexesOnly;
      const numStr = minIdx.toString();
      if (str.substr(i, numStr.length) === numStr && str[i + numStr.length] === '.') {
        pos = i;
        break;
      }
    }
    if (pos !== -1) {
      const dotPos = pos + minIdx.toString().length;
    }
    const nDigits = n;
    let underline = "";
    for (let i = 0; i < n; i++) {
      if (i === minIdx) {
        underline += "\\__";
      } else if (i === minIdx + 1) {
        underline += "__/";
      } else {
      }
    }
    function getIndexPosition(idx) {
      let pos = 0;
      for (let i = 0; i < idx; i++) {
        const numStr = i.toString();
        pos += numStr.length + 1;
        if (i < n-1) pos += 2;
      }
      return pos;
    }
    const startIdxPos = getIndexPosition(minIdx);
    const secondIdxPos = getIndexPosition(minIdx+1);
    const underlineChars = Array(indexesOnly.length).fill(' ');
    for (let i = 0; i < 3; i++) {
      if (startIdxPos + i < underlineChars.length) {
        const pattern = "\\__";
        underlineChars[startIdxPos + i] = pattern[i];
      }
    }
    for (let i = 0; i < 3; i++) {
      if (secondIdxPos + i < underlineChars.length) {
        const pattern = "__/";
        underlineChars[secondIdxPos + i] = pattern[i];
      }
    }
    if (startIdxPos + 3 < underlineChars.length) {
      underlineChars[startIdxPos + 3] = ' ';
    }
    return prefix + underlineChars.join('');
  }
  function generateOutput() {
    const n = 20;
    const data = gen_random_vector(n);
    const { index: minIdx, sum: minSum } = min_adjacent_sum(data);
    const val1 = data[minIdx];
    const val2 = data[minIdx+1];
    const indexesLine = formatIndexesLine(n);
    const dataLine = formatDataLine(data);
    let underlineLine;
    try {
      underlineLine = formatUnderlineLine(n, minIdx);
    } catch(e) {
      underlineLine = "indexes:                    \\__ __/      (approx)";
    }

    const resultLine = `min adjacent sum=${val1}+${val2}=${minSum} at indexes:${minIdx},${minIdx+1}`;

    return `${indexesLine}\n${dataLine}\n${underlineLine}\n${resultLine}`;
  }
  function runAndDisplay() {
    const output = generateOutput();
    console.log(output);
    document.getElementById("output").textContent = output;
  }
  runAndDisplay();
</script>
</body>
</html>
