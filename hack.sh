pdfinfo $1 | awk -F' ' '/Page/ && 6 == NF { printf "%dp %dp", $3, $5 }'
