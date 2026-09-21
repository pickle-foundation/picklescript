Ident 0:4 text="test"
Fn 5:7
Ident 8:26 text="bytes_empty_string"
LParen 26:27
RParen 27:28
LBrace 29:30
Newline 30:31
Let 35:38
Ident 39:40 text="b"
Assign 41:42
Ident 43:48 text="bytes"
LParen 48:49
Str 49:51
RParen 51:52
Newline 52:53
Ident 57:63 text="expect"
LParen 63:64
Ident 64:67 text="len"
LParen 67:68
Ident 68:69 text="b"
RParen 69:70
RParen 70:71
Dot 71:72
Ident 72:76 text="toBe"
LParen 76:77
Number 77:78 text="0" suf="" isf=0 int=0
RParen 78:79
Newline 79:80
Ident 84:90 text="expect"
LParen 90:91
Ident 91:94 text="str"
LParen 94:95
Ident 95:96 text="b"
RParen 96:97
RParen 97:98
Dot 98:99
Ident 99:103 text="toBe"
LParen 103:104
Str 104:106
RParen 106:107
Newline 107:108
RBrace 108:109
Newline 109:110
Newline 110:111
Ident 111:115 text="test"
Fn 116:118
Ident 119:140 text="bytes_ascii_roundtrip"
LParen 140:141
RParen 141:142
LBrace 143:144
Newline 144:145
Let 149:152
Ident 153:154 text="s"
Assign 155:156
Str 157:161 T("hi")
Newline 161:162
Let 166:169
Ident 170:171 text="b"
Assign 172:173
Ident 174:179 text="bytes"
LParen 179:180
Ident 180:181 text="s"
RParen 181:182
Newline 182:183
Ident 187:193 text="expect"
LParen 193:194
Ident 194:197 text="len"
LParen 197:198
Ident 198:199 text="b"
RParen 199:200
RParen 200:201
Dot 201:202
Ident 202:206 text="toBe"
LParen 206:207
Number 207:208 text="2" suf="" isf=0 int=2
RParen 208:209
Newline 209:210
Ident 214:220 text="expect"
LParen 220:221
Ident 221:222 text="b"
LBracket 222:223
Number 223:224 text="0" suf="" isf=0 int=0
RBracket 224:225
EqEq 226:228
Number 229:232 text="104" suf="" isf=0 int=104
RParen 232:233
Dot 233:234
Ident 234:238 text="toBe"
LParen 238:239
True 239:243
RParen 243:244
Newline 244:245
Ident 249:255 text="expect"
LParen 255:256
Ident 256:257 text="b"
LBracket 257:258
Number 258:259 text="1" suf="" isf=0 int=1
RBracket 259:260
EqEq 261:263
Number 264:267 text="105" suf="" isf=0 int=105
RParen 267:268
Dot 268:269
Ident 269:273 text="toBe"
LParen 273:274
True 274:278
RParen 278:279
Newline 279:280
Ident 284:290 text="expect"
LParen 290:291
Ident 291:294 text="str"
LParen 294:295
Ident 295:296 text="b"
RParen 296:297
RParen 297:298
Dot 298:299
Ident 299:303 text="toBe"
LParen 303:304
Str 304:308 T("hi")
RParen 308:309
Newline 309:310
RBrace 310:311
Newline 311:312
Newline 312:313
Ident 313:317 text="test"
Fn 318:320
Ident 321:351 text="bytes_utf8_multibyte_roundtrip"
LParen 351:352
RParen 352:353
LBrace 354:355
Newline 355:356
Let 360:363
Ident 364:365 text="s"
Assign 366:367
Str 368:372 T("é")
Newline 372:373
Let 377:380
Ident 381:382 text="b"
Assign 383:384
Ident 385:390 text="bytes"
LParen 390:391
Ident 391:392 text="s"
RParen 392:393
Newline 393:394
Ident 398:404 text="expect"
LParen 404:405
Ident 405:408 text="len"
LParen 408:409
Ident 409:410 text="b"
RParen 410:411
RParen 411:412
Dot 412:413
Ident 413:417 text="toBe"
LParen 417:418
Number 418:419 text="2" suf="" isf=0 int=2
RParen 419:420
Newline 420:421
Ident 425:431 text="expect"
LParen 431:432
Ident 432:433 text="b"
LBracket 433:434
Number 434:435 text="0" suf="" isf=0 int=0
RBracket 435:436
EqEq 437:439
Number 440:443 text="195" suf="" isf=0 int=195
RParen 443:444
Dot 444:445
Ident 445:449 text="toBe"
LParen 449:450
True 450:454
RParen 454:455
Newline 455:456
Ident 460:466 text="expect"
LParen 466:467
Ident 467:468 text="b"
LBracket 468:469
Number 469:470 text="1" suf="" isf=0 int=1
RBracket 470:471
EqEq 472:474
Number 475:478 text="169" suf="" isf=0 int=169
RParen 478:479
Dot 479:480
Ident 480:484 text="toBe"
LParen 484:485
True 485:489
RParen 489:490
Newline 490:491
Ident 495:501 text="expect"
LParen 501:502
Ident 502:505 text="str"
LParen 505:506
Ident 506:507 text="b"
RParen 507:508
RParen 508:509
Dot 509:510
Ident 510:514 text="toBe"
LParen 514:515
Str 515:519 T("é")
RParen 519:520
Newline 520:521
RBrace 521:522
Newline 522:523
Newline 523:524
Ident 524:528 text="test"
Fn 529:531
Ident 532:564 text="bytes_matches_string_index_reads"
LParen 564:565
RParen 565:566
LBrace 567:568
Newline 568:569
Let 573:576
Ident 577:578 text="s"
Assign 579:580
Str 581:588 T("café")
Newline 588:589
Let 593:596
Ident 597:598 text="b"
Assign 599:600
Ident 601:606 text="bytes"
LParen 606:607
Ident 607:608 text="s"
RParen 608:609
Newline 609:610
For 614:617
LParen 618:619
Ident 619:620 text="i"
In 621:623
Number 624:625 text="0" suf="" isf=0 int=0
Range 625:627
Ident 627:630 text="len"
LParen 630:631
Ident 631:632 text="s"
RParen 632:633
RParen 633:634
LBrace 635:636
Newline 636:637
Ident 645:651 text="expect"
LParen 651:652
Ident 652:653 text="b"
LBracket 653:654
Ident 654:655 text="i"
RBracket 655:656
RParen 656:657
Dot 657:658
Ident 658:662 text="toBe"
LParen 662:663
Ident 663:664 text="s"
LBracket 664:665
Ident 665:666 text="i"
RBracket 666:667
RParen 667:668
Newline 668:669
RBrace 673:674
Newline 674:675
Ident 679:685 text="expect"
LParen 685:686
Ident 686:689 text="len"
LParen 689:690
Ident 690:691 text="b"
RParen 691:692
RParen 692:693
Dot 693:694
Ident 694:698 text="toBe"
LParen 698:699
Ident 699:702 text="len"
LParen 702:703
Ident 703:704 text="s"
RParen 704:705
RParen 705:706
Newline 706:707
RBrace 707:708
Newline 708:709
Newline 709:710
Ident 710:714 text="test"
Fn 715:717
Ident 718:751 text="str_bytes_reverse_keeps_all_bytes"
LParen 751:752
RParen 752:753
LBrace 754:755
Newline 755:756
Let 760:763
Ident 764:765 text="s"
Assign 766:767
Str 768:775 T("a\0bc")
Newline 775:776
Let 780:783
Ident 784:785 text="b"
Assign 786:787
Ident 788:793 text="bytes"
LParen 793:794
Ident 794:795 text="s"
RParen 795:796
Newline 796:797
Ident 801:807 text="expect"
LParen 807:808
Ident 808:811 text="len"
LParen 811:812
Ident 812:813 text="b"
RParen 813:814
RParen 814:815
Dot 815:816
Ident 816:820 text="toBe"
LParen 820:821
Number 821:822 text="4" suf="" isf=0 int=4
RParen 822:823
Newline 823:824
Ident 828:834 text="expect"
LParen 834:835
Ident 835:836 text="b"
LBracket 836:837
Number 837:838 text="0" suf="" isf=0 int=0
RBracket 838:839
EqEq 840:842
Number 843:845 text="97" suf="" isf=0 int=97
RParen 845:846
Dot 846:847
Ident 847:851 text="toBe"
LParen 851:852
True 852:856
RParen 856:857
Newline 857:858
Ident 862:868 text="expect"
LParen 868:869
Ident 869:870 text="b"
LBracket 870:871
Number 871:872 text="1" suf="" isf=0 int=1
RBracket 872:873
EqEq 874:876
Number 877:878 text="0" suf="" isf=0 int=0
RParen 878:879
Dot 879:880
Ident 880:884 text="toBe"
LParen 884:885
True 885:889
RParen 889:890
Newline 890:891
Ident 895:901 text="expect"
LParen 901:902
Ident 902:905 text="str"
LParen 905:906
Ident 906:907 text="b"
RParen 907:908
RParen 908:909
Dot 909:910
Ident 910:914 text="toBe"
LParen 914:915
Ident 915:916 text="s"
RParen 916:917
Newline 917:918
RBrace 918:919
Newline 919:920
Newline 920:921
Ident 921:925 text="test"
Fn 926:928
Ident 929:955 text="str_bytes_inverse_of_bytes"
LParen 955:956
RParen 956:957
LBrace 958:959
Newline 959:960
Let 964:967
Ident 968:969 text="s"
Assign 970:971
Str 972:984 T("round-trip")
Newline 984:985
Ident 989:995 text="expect"
LParen 995:996
Ident 996:999 text="str"
LParen 999:1000
Ident 1000:1005 text="bytes"
LParen 1005:1006
Ident 1006:1007 text="s"
RParen 1007:1008
RParen 1008:1009
RParen 1009:1010
Dot 1010:1011
Ident 1011:1015 text="toBe"
LParen 1015:1016
Ident 1016:1017 text="s"
RParen 1017:1018
Newline 1018:1019
RBrace 1019:1020
Newline 1020:1021
Newline 1021:1022
Ident 1022:1026 text="test"
Fn 1027:1029
Ident 1030:1048 text="bytes_iterate_sums"
LParen 1048:1049
RParen 1049:1050
LBrace 1051:1052
Newline 1052:1053
Let 1057:1060
Ident 1061:1062 text="s"
Assign 1063:1064
Str 1065:1070 T("abc")
Newline 1070:1071
Var 1075:1078
Ident 1079:1080 text="n"
Assign 1081:1082
Number 1083:1084 text="0" suf="" isf=0 int=0
Newline 1084:1085
For 1089:1092
LParen 1093:1094
Ident 1094:1095 text="b"
In 1096:1098
Ident 1099:1104 text="bytes"
LParen 1104:1105
Ident 1105:1106 text="s"
RParen 1106:1107
RParen 1107:1108
LBrace 1109:1110
Newline 1110:1111
If 1119:1121
LParen 1122:1123
Ident 1123:1124 text="b"
EqEq 1125:1127
Number 1128:1130 text="98" suf="" isf=0 int=98
RParen 1130:1131
LBrace 1132:1133
Newline 1133:1134
Ident 1146:1147 text="n"
PlusEq 1148:1150
Number 1151:1152 text="1" suf="" isf=0 int=1
Newline 1152:1153
RBrace 1161:1162
Newline 1162:1163
RBrace 1167:1168
Newline 1168:1169
Ident 1173:1179 text="expect"
LParen 1179:1180
Ident 1180:1181 text="n"
RParen 1181:1182
Dot 1182:1183
Ident 1183:1187 text="toBe"
LParen 1187:1188
Number 1188:1189 text="1" suf="" isf=0 int=1
RParen 1189:1190
Newline 1190:1191
RBrace 1191:1192
Eof 1192:1192
