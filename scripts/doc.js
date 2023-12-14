const fs = require("fs");
const path = require("path");

const problemsDirectory = "src/problems";

function getProblemLinks(directory) {
  const links = [];
  const files = fs.readdirSync(directory);

  files.forEach((file) => {
    const filePath = path.join(directory, file);
    const stat = fs.statSync(filePath);

    if (stat.isDirectory()) {
      links.push(...getProblemLinks(filePath));
    } else if (path.extname(file) === ".rs") {
      const relativePath = path.relative(__dirname, filePath);
      links.push(`- [${file}](${relativePath})`);
    }
  });

  return links;
}

const mdContent = getProblemLinks(problemsDirectory).join("\n");

const outputFilePath = "leetcode_problems.md";
fs.writeFileSync(outputFilePath, mdContent);

console.log(`Markdown file generated at: ${outputFilePath}`);
