Ident 0:4 text="test"
Fn 5:7
Ident 8:27 text="str_set_int_literal"
LParen 27:28
RParen 28:29
LBrace 30:31
Newline 31:32
Let 36:39
Ident 40:41 text="s"
Assign 42:43
Str 44:52 T("abcdef")
Newline 52:53
Ident 57:58 text="s"
LBracket 58:59
Number 59:60 text="0" suf="" isf=0 int=0
RBracket 60:61
Assign 62:63
Number 64:66 text="65" suf="" isf=0 int=65
Newline 66:67
Ident 71:72 text="s"
LBracket 72:73
Number 73:74 text="5" suf="" isf=0 int=5
RBracket 74:75
Assign 76:77
Number 78:80 text="33" suf="" isf=0 int=33
Newline 80:81
Ident 85:91 text="expect"
LParen 91:92
Ident 92:93 text="s"
RParen 93:94
Dot 94:95
Ident 95:99 text="toBe"
LParen 99:100
Str 100:108 T("Abcde!")
RParen 108:109
Newline 109:110
Ident 114:120 text="expect"
LParen 120:121
Ident 121:122 text="s"
LBracket 122:123
Number 123:124 text="0" suf="" isf=0 int=0
RBracket 124:125
EqEq 126:128
Number 129:131 text="65" suf="" isf=0 int=65
RParen 131:132
Dot 132:133
Ident 133:137 text="toBe"
LParen 137:138
True 138:142
RParen 142:143
Newline 143:144
RBrace 144:145
Newline 145:146
Newline 146:147
Ident 147:151 text="test"
Fn 152:154
Ident 155:175 text="str_set_char_literal"
LParen 175:176
RParen 176:177
LBrace 178:179
Newline 179:180
Let 184:187
Ident 188:189 text="s"
Assign 190:191
Str 192:200 T("abcdef")
Newline 200:201
Ident 205:206 text="s"
LBracket 206:207
Number 207:208 text="1" suf="" isf=0 int=1
RBracket 208:209
Assign 210:211
Char 212:215 ch='Z' int=90
Newline 215:216
Ident 220:226 text="expect"
LParen 226:227
Ident 227:228 text="s"
RParen 228:229
Dot 229:230
Ident 230:234 text="toBe"
LParen 234:235
Str 235:243 T("aZcdef")
RParen 243:244
Newline 244:245
Ident 249:250 text="s"
LBracket 250:251
Number 251:252 text="3" suf="" isf=0 int=3
RBracket 252:253
Assign 254:255
Char 256:259 ch='3' int=51
Newline 259:260
Ident 264:270 text="expect"
LParen 270:271
Ident 271:272 text="s"
RParen 272:273
Dot 273:274
Ident 274:278 text="toBe"
LParen 278:279
Str 279:287 T("aZc3ef")
RParen 287:288
Newline 288:289
RBrace 289:290
Newline 290:291
Newline 291:292
Ident 292:296 text="test"
Fn 297:299
Ident 300:316 text="str_set_compound"
LParen 316:317
RParen 317:318
LBrace 319:320
Newline 320:321
Let 325:328
Ident 329:330 text="s"
Assign 331:332
Str 333:338 T("a1f")
Newline 338:339
Ident 343:344 text="s"
LBracket 344:345
Number 345:346 text="0" suf="" isf=0 int=0
RBracket 346:347
PlusEq 348:350
Number 351:352 text="1" suf="" isf=0 int=1
Newline 352:353
Ident 357:363 text="expect"
LParen 363:364
Ident 364:365 text="s"
RParen 365:366
Dot 366:367
Ident 367:371 text="toBe"
LParen 371:372
Str 372:377 T("b1f")
RParen 377:378
Newline 378:379
Ident 383:384 text="s"
LBracket 384:385
Number 385:386 text="2" suf="" isf=0 int=2
RBracket 386:387
MinusEq 388:390
Number 391:393 text="32" suf="" isf=0 int=32
Newline 393:394
Ident 398:404 text="expect"
LParen 404:405
Ident 405:406 text="s"
RParen 406:407
Dot 407:408
Ident 408:412 text="toBe"
LParen 412:413
Str 413:418 T("b1F")
RParen 418:419
Newline 419:420
RBrace 420:421
Newline 421:422
Newline 422:423
Ident 423:427 text="test"
Fn 428:430
Ident 431:449 text="str_set_byte_value"
LParen 449:450
RParen 450:451
LBrace 452:453
Newline 453:454
Let 458:461
Ident 462:463 text="s"
Assign 464:465
Str 466:473 T("hello")
Newline 473:474
Let 478:481
Ident 482:483 text="b"
Assign 484:485
Ident 486:487 text="s"
LBracket 487:488
Number 488:489 text="0" suf="" isf=0 int=0
RBracket 489:490
Newline 490:491
Ident 495:496 text="s"
LBracket 496:497
Number 497:498 text="1" suf="" isf=0 int=1
RBracket 498:499
Assign 500:501
Ident 502:503 text="b"
Newline 503:504
Ident 508:514 text="expect"
LParen 514:515
Ident 515:516 text="s"
RParen 516:517
Dot 517:518
Ident 518:522 text="toBe"
LParen 522:523
Str 523:530 T("hhllo")
RParen 530:531
Newline 531:532
RBrace 532:533
Newline 533:534
Newline 534:535
Ident 535:539 text="test"
Fn 540:542
Ident 543:564 text="str_set_copy_on_write"
LParen 564:565
RParen 565:566
LBrace 567:568
Newline 568:569
Let 573:576
Ident 577:578 text="s"
Assign 579:580
Str 581:586 T("abc")
Newline 586:587
Let 591:594
Ident 595:596 text="t"
Assign 597:598
Ident 599:600 text="s"
Newline 600:601
Ident 605:606 text="s"
LBracket 606:607
Number 607:608 text="0" suf="" isf=0 int=0
RBracket 608:609
Assign 610:611
Char 612:615 ch='x' int=120
Newline 615:616
Ident 620:626 text="expect"
LParen 626:627
Ident 627:628 text="s"
RParen 628:629
Dot 629:630
Ident 630:634 text="toBe"
LParen 634:635
Str 635:640 T("xbc")
RParen 640:641
Newline 641:642
Ident 646:652 text="expect"
LParen 652:653
Ident 653:654 text="t"
RParen 654:655
Dot 655:656
Ident 656:660 text="toBe"
LParen 660:661
Str 661:666 T("abc")
RParen 666:667
Newline 667:668
RBrace 668:669
Newline 669:670
Newline 670:671
Ident 671:675 text="test"
Fn 676:678
Ident 679:704 text="str_set_keeps_alias_reads"
LParen 704:705
RParen 705:706
LBrace 707:708
Newline 708:709
Let 713:716
Ident 717:718 text="s"
Assign 719:720
Str 721:729 T("abcdef")
Newline 729:730
Let 734:737
Ident 738:739 text="t"
Assign 740:741
Ident 742:743 text="s"
Newline 743:744
Ident 748:749 text="s"
LBracket 749:750
Number 750:751 text="2" suf="" isf=0 int=2
RBracket 751:752
Assign 753:754
Char 755:758 ch='X' int=88
Newline 758:759
Ident 763:769 text="expect"
LParen 769:770
Ident 770:773 text="len"
LParen 773:774
Ident 774:775 text="s"
RParen 775:776
RParen 776:777
Dot 777:778
Ident 778:782 text="toBe"
LParen 782:783
Number 783:784 text="6" suf="" isf=0 int=6
RParen 784:785
Newline 785:786
Ident 790:796 text="expect"
LParen 796:797
Ident 797:798 text="s"
LBracket 798:799
Number 799:800 text="2" suf="" isf=0 int=2
RBracket 800:801
EqEq 802:804
Number 805:807 text="88" suf="" isf=0 int=88
RParen 807:808
Dot 808:809
Ident 809:813 text="toBe"
LParen 813:814
True 814:818
RParen 818:819
Newline 819:820
Ident 824:830 text="expect"
LParen 830:831
Ident 831:832 text="t"
LBracket 832:833
Number 833:834 text="2" suf="" isf=0 int=2
RBracket 834:835
EqEq 836:838
Number 839:841 text="99" suf="" isf=0 int=99
RParen 841:842
Dot 842:843
Ident 843:847 text="toBe"
LParen 847:848
True 848:852
RParen 852:853
Newline 853:854
RBrace 854:855
Newline 855:856
Newline 856:857
Class 857:862
Ident 863:875 text="StrSetHolder"
LBrace 876:877
Newline 877:878
Var 882:885
Ident 886:890 text="text"
Assign 891:892
Str 893:900 T("world")
Newline 900:901
Static 905:911
Var 912:915
Ident 916:920 text="sbuf"
Assign 921:922
Str 923:930 T("hello")
Newline 930:931
Fn 935:937
Ident 938:942 text="bump"
LParen 942:943
RParen 943:944
LBrace 945:946
Newline 946:947
Ident 955:959 text="text"
LBracket 959:960
Number 960:961 text="0" suf="" isf=0 int=0
RBracket 961:962
Assign 963:964
Char 965:968 ch='W' int=87
Newline 968:969
Ident 977:981 text="sbuf"
LBracket 981:982
Number 982:983 text="1" suf="" isf=0 int=1
RBracket 983:984
Assign 985:986
Char 987:990 ch='E' int=69
Newline 990:991
RBrace 995:996
Newline 996:997
RBrace 997:998
Newline 998:999
Newline 999:1000
Ident 1000:1004 text="test"
Fn 1005:1007
Ident 1008:1029 text="str_set_field_targets"
LParen 1029:1030
RParen 1030:1031
LBrace 1032:1033
Newline 1033:1034
Let 1038:1041
Ident 1042:1043 text="d"
Assign 1044:1045
Ident 1046:1058 text="StrSetHolder"
LParen 1058:1059
RParen 1059:1060
Newline 1060:1061
Ident 1065:1066 text="d"
Dot 1066:1067
Ident 1067:1071 text="text"
LBracket 1071:1072
Number 1072:1073 text="3" suf="" isf=0 int=3
RBracket 1073:1074
Assign 1075:1076
Char 1077:1080 ch='L' int=76
Newline 1080:1081
Ident 1085:1091 text="expect"
LParen 1091:1092
Ident 1092:1093 text="d"
Dot 1093:1094
Ident 1094:1098 text="text"
RParen 1098:1099
Dot 1099:1100
Ident 1100:1104 text="toBe"
LParen 1104:1105
Str 1105:1112 T("worLd")
RParen 1112:1113
Newline 1113:1114
Ident 1118:1119 text="d"
Dot 1119:1120
Ident 1120:1124 text="bump"
LParen 1124:1125
RParen 1125:1126
Newline 1126:1127
Ident 1131:1137 text="expect"
LParen 1137:1138
Ident 1138:1139 text="d"
Dot 1139:1140
Ident 1140:1144 text="text"
RParen 1144:1145
Dot 1145:1146
Ident 1146:1150 text="toBe"
LParen 1150:1151
Str 1151:1158 T("WorLd")
RParen 1158:1159
Newline 1159:1160
Ident 1164:1170 text="expect"
LParen 1170:1171
Ident 1171:1183 text="StrSetHolder"
Dot 1183:1184
Ident 1184:1188 text="sbuf"
RParen 1188:1189
Dot 1189:1190
Ident 1190:1194 text="toBe"
LParen 1194:1195
Str 1195:1202 T("hEllo")
RParen 1202:1203
Newline 1203:1204
RBrace 1204:1205
Newline 1205:1206
Newline 1206:1207
Ident 1207:1211 text="test"
Fn 1212:1214
Ident 1215:1239 text="str_set_local_write_back"
LParen 1239:1240
RParen 1240:1241
LBrace 1242:1243
Newline 1243:1244
Var 1248:1251
Ident 1252:1253 text="v"
Assign 1254:1255
Str 1256:1261 T("abc")
Newline 1261:1262
Ident 1266:1267 text="v"
LBracket 1267:1268
Number 1268:1269 text="1" suf="" isf=0 int=1
RBracket 1269:1270
Assign 1271:1272
Char 1273:1276 ch='Y' int=89
Newline 1276:1277
Ident 1281:1287 text="expect"
LParen 1287:1288
Ident 1288:1289 text="v"
RParen 1289:1290
Dot 1290:1291
Ident 1291:1295 text="toBe"
LParen 1295:1296
Str 1296:1301 T("aYc")
RParen 1301:1302
Newline 1302:1303
RBrace 1303:1304
Eof 1304:1304
