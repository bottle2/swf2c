#!/usr/bin/perl
sub cell { print sprintf("%.2f%s", (-s "$ARGV[0]$_[0]") / 1000000.0) =~ s/\./\\&,/r, "\t" x $_[1] }
cell(".swf",1);
cell(".js",1);
cell(".js.gz",1);
cell(".js.br",1);
cell(".js.zst",2);
cell("_360.mp4",1);
cell("_720.mp4",1);
cell("_1080.mp4",1);
cell("_4k.mp4",0);
