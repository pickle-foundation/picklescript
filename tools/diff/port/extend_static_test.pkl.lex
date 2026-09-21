Class 0:5
Ident 6:11 text="Store"
LBrace 12:13
Newline 13:14
Static 18:24
Var 25:28
Ident 29:34 text="count"
Colon 34:35
Ident 36:39 text="int"
Assign 40:41
Number 42:43 text="1" suf="" isf=0 int=1
Newline 43:44
Static 48:54
Var 55:58
Ident 59:62 text="val"
Colon 62:63
Ident 64:67 text="int"
Assign 68:69
Number 70:72 text="10" suf="" isf=0 int=10
Newline 72:73
Const 77:82
Ident 83:88 text="LIMIT"
Colon 88:89
Ident 90:93 text="int"
Assign 94:95
Number 96:98 text="99" suf="" isf=0 int=99
Newline 98:99
RBrace 99:100
Newline 100:101
Newline 101:102
Class 102:107
Ident 108:117 text="Warehouse"
Extends 118:125
Ident 126:131 text="Store"
LBrace 132:133
Newline 133:134
Static 138:144
Var 145:148
Ident 149:154 text="count"
Colon 154:155
Ident 156:159 text="int"
Assign 160:161
Number 162:163 text="5" suf="" isf=0 int=5
Newline 163:164
Static 168:174
Var 175:178
Ident 179:184 text="bonus"
Colon 184:185
Ident 186:189 text="int"
Assign 190:191
Number 192:193 text="7" suf="" isf=0 int=7
Newline 193:194
RBrace 194:195
Newline 195:196
Newline 196:197
Class 197:202
Ident 203:209 text="Animal"
LBrace 210:211
Newline 211:212
Fn 216:218
Ident 219:224 text="speak"
LParen 224:225
RParen 225:226
Arrow 227:229
Ident 230:236 text="string"
LBrace 237:238
Newline 238:239
Return 247:253
Str 254:263 T("generic")
Newline 263:264
RBrace 268:269
Newline 269:270
RBrace 270:271
Newline 271:272
Newline 272:273
Class 273:278
Ident 279:282 text="Dog"
Extends 283:290
Ident 291:297 text="Animal"
LBrace 298:299
Newline 299:300
Override 304:312
Fn 313:315
Ident 316:321 text="speak"
LParen 321:322
RParen 322:323
Arrow 324:326
Ident 327:333 text="string"
LBrace 334:335
Newline 335:336
Return 344:350
Str 351:357 T("woof")
Newline 357:358
RBrace 362:363
Newline 363:364
RBrace 364:365
Newline 365:366
Newline 366:367
Ident 367:375 text="describe"
LParen 375:376
Str 376:406 T("inherited statics and consts")
Comma 406:407
LBrace 408:409
Newline 409:410
Ident 414:418 text="test"
LParen 418:419
Str 419:442 T("own static field wins")
Comma 442:443
LBrace 444:445
Newline 445:446
Ident 454:460 text="expect"
LParen 460:461
Ident 461:466 text="Store"
Dot 466:467
Ident 467:472 text="count"
RParen 472:473
Dot 473:474
Ident 474:478 text="toBe"
LParen 478:479
Number 479:480 text="1" suf="" isf=0 int=1
RParen 480:481
Newline 481:482
Ident 490:496 text="expect"
LParen 496:497
Ident 497:506 text="Warehouse"
Dot 506:507
Ident 507:512 text="count"
RParen 512:513
Dot 513:514
Ident 514:518 text="toBe"
LParen 518:519
Number 519:520 text="5" suf="" isf=0 int=5
RParen 520:521
Newline 521:522
RBrace 526:527
RParen 527:528
Newline 528:529
Newline 529:530
Ident 534:538 text="test"
LParen 538:539
Str 539:594 T("inherited static field resolves to the declaring cell")
Comma 594:595
LBrace 596:597
Newline 597:598
Ident 606:612 text="expect"
LParen 612:613
Ident 613:622 text="Warehouse"
Dot 622:623
Ident 623:626 text="val"
RParen 626:627
Dot 627:628
Ident 628:632 text="toBe"
LParen 632:633
Number 633:635 text="10" suf="" isf=0 int=10
RParen 635:636
Newline 636:637
Ident 645:654 text="Warehouse"
Dot 654:655
Ident 655:658 text="val"
Assign 659:660
Number 661:663 text="30" suf="" isf=0 int=30
Newline 663:664
Ident 672:678 text="expect"
LParen 678:679
Ident 679:684 text="Store"
Dot 684:685
Ident 685:688 text="val"
RParen 688:689
Dot 689:690
Ident 690:694 text="toBe"
LParen 694:695
Number 695:697 text="30" suf="" isf=0 int=30
RParen 697:698
Newline 698:699
Ident 707:713 text="expect"
LParen 713:714
Ident 714:723 text="Warehouse"
Dot 723:724
Ident 724:727 text="val"
RParen 727:728
Dot 728:729
Ident 729:733 text="toBe"
LParen 733:734
Number 734:736 text="30" suf="" isf=0 int=30
RParen 736:737
Newline 737:738
RBrace 742:743
RParen 743:744
Newline 744:745
Newline 745:746
Ident 750:754 text="test"
LParen 754:755
Str 755:781 T("inherited const resolves")
Comma 781:782
LBrace 783:784
Newline 784:785
Ident 793:799 text="expect"
LParen 799:800
Ident 800:809 text="Warehouse"
Dot 809:810
Ident 810:815 text="LIMIT"
RParen 815:816
Dot 816:817
Ident 817:821 text="toBe"
LParen 821:822
Number 822:824 text="99" suf="" isf=0 int=99
RParen 824:825
Newline 825:826
Ident 834:840 text="expect"
LParen 840:841
Ident 841:846 text="Store"
Dot 846:847
Ident 847:852 text="LIMIT"
RParen 852:853
Dot 853:854
Ident 854:858 text="toBe"
LParen 858:859
Number 859:861 text="99" suf="" isf=0 int=99
RParen 861:862
Newline 862:863
RBrace 867:868
RParen 868:869
Newline 869:870
Newline 870:871
Ident 875:879 text="test"
LParen 879:880
Str 880:914 T("own static field after inherited")
Comma 914:915
LBrace 916:917
Newline 917:918
Ident 926:932 text="expect"
LParen 932:933
Ident 933:942 text="Warehouse"
Dot 942:943
Ident 943:948 text="bonus"
RParen 948:949
Dot 949:950
Ident 950:954 text="toBe"
LParen 954:955
Number 955:956 text="7" suf="" isf=0 int=7
RParen 956:957
Newline 957:958
RBrace 962:963
RParen 963:964
Newline 964:965
RBrace 965:966
RParen 966:967
Newline 967:968
Newline 968:969
Ident 969:977 text="describe"
LParen 977:978
Str 978:1019 T("overridden methods used as bound values")
Comma 1019:1020
LBrace 1021:1022
Newline 1022:1023
Ident 1027:1031 text="test"
LParen 1031:1032
Str 1032:1077 T("plain receiver uses the base implementation")
Comma 1077:1078
LBrace 1079:1080
Newline 1080:1081
Let 1089:1092
Ident 1093:1094 text="f"
Colon 1094:1095
Fn 1096:1098
LParen 1099:1100
RParen 1100:1101
Arrow 1102:1104
Ident 1105:1111 text="string"
Assign 1112:1113
Ident 1114:1120 text="Animal"
LParen 1120:1121
RParen 1121:1122
Dot 1122:1123
Ident 1123:1128 text="speak"
Newline 1128:1129
Ident 1137:1143 text="expect"
LParen 1143:1144
Ident 1144:1145 text="f"
LParen 1145:1146
RParen 1146:1147
RParen 1147:1148
Dot 1148:1149
Ident 1149:1153 text="toBe"
LParen 1153:1154
Str 1154:1163 T("generic")
RParen 1163:1164
Newline 1164:1165
RBrace 1169:1170
RParen 1170:1171
Newline 1171:1172
Newline 1172:1173
Ident 1177:1181 text="test"
LParen 1181:1182
Str 1182:1224 T("overridden receiver dispatches virtually")
Comma 1224:1225
LBrace 1226:1227
Newline 1227:1228
Let 1236:1239
Ident 1240:1243 text="dog"
Colon 1243:1244
Ident 1245:1251 text="Animal"
Assign 1252:1253
Ident 1254:1257 text="Dog"
LParen 1257:1258
RParen 1258:1259
Newline 1259:1260
Let 1268:1271
Ident 1272:1273 text="g"
Assign 1274:1275
Ident 1276:1279 text="dog"
Dot 1279:1280
Ident 1280:1285 text="speak"
Newline 1285:1286
Ident 1294:1300 text="expect"
LParen 1300:1301
Ident 1301:1302 text="g"
LParen 1302:1303
RParen 1303:1304
RParen 1304:1305
Dot 1305:1306
Ident 1306:1310 text="toBe"
LParen 1310:1311
Str 1311:1317 T("woof")
RParen 1317:1318
Newline 1318:1319
Newline 1319:1320
Let 1328:1331
Ident 1332:1333 text="h"
Assign 1334:1335
Ident 1336:1342 text="Animal"
LParen 1342:1343
RParen 1343:1344
Dot 1344:1345
Ident 1345:1350 text="speak"
Newline 1350:1351
Ident 1359:1365 text="expect"
LParen 1365:1366
Ident 1366:1367 text="h"
LParen 1367:1368
RParen 1368:1369
RParen 1369:1370
Dot 1370:1371
Ident 1371:1375 text="toBe"
LParen 1375:1376
Str 1376:1385 T("generic")
RParen 1385:1386
Newline 1386:1387
RBrace 1391:1392
RParen 1392:1393
Newline 1393:1394
RBrace 1394:1395
RParen 1395:1396
Eof 1396:1396
