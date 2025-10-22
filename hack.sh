pdfinfo $1.pdf | awk -F' ' '/Page/ && 6 == NF { printf "%dp %dp", $3, $5 }'
