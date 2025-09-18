sed "s/@aq@/'/g" | sed "s/@co@/,/g" | column -s ',' -t -o ',' | sed 's/)\( *\),/\1),/'
# Thanks DPA from #c @ Libera.Chat
