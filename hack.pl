#!/usr/bin/perl
sub cell { print sprintf("%.2f%s", (-s "$ARGV[0]$_[0]") / 1000000.0) =~ s/\./\\&,/r, "\t" x $_[1] }
cell(".swf",5);
cell(".js.gz",1);
cell(".js.br",1);
cell(".js.zst",0);
