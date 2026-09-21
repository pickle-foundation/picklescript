Ident 0:4 text="test"
Fn 5:7
Ident 8:27 text="map_get_present_int"
LParen 27:28
RParen 28:29
LBrace 30:31
Newline 31:32
Let 36:39
Ident 40:41 text="m"
Assign 42:43
LBrace 44:45
Str 45:48 T("a")
Colon 48:49
Number 50:51 text="1" suf="" isf=0 int=1
Comma 51:52
Str 53:56 T("b")
Colon 56:57
Number 58:59 text="2" suf="" isf=0 int=2
RBrace 59:60
Newline 60:61
Let 65:68
Ident 69:72 text="got"
Assign 73:74
Ident 75:76 text="m"
Dot 76:77
Get 77:80
LParen 80:81
Str 81:84 T("a")
RParen 84:85
Newline 85:86
If 90:92
LParen 93:94
Let 94:97
Ident 98:102 text="some"
LParen 102:103
Ident 103:104 text="v"
RParen 104:105
Assign 106:107
Ident 108:111 text="got"
RParen 111:112
LBrace 113:114
Newline 114:115
Ident 123:129 text="expect"
LParen 129:130
Ident 130:131 text="v"
RParen 131:132
Dot 132:133
Ident 133:137 text="toBe"
LParen 137:138
Number 138:139 text="1" suf="" isf=0 int=1
RParen 139:140
Newline 140:141
RBrace 145:146
Else 147:151
LBrace 152:153
Newline 153:154
Ident 162:168 text="expect"
LParen 168:169
Str 169:181 T("unexpected")
RParen 181:182
Dot 182:183
Ident 183:187 text="toBe"
LParen 187:188
Str 188:246 T("map_get_present_int: get returned none for a present key")
RParen 246:247
Newline 247:248
RBrace 252:253
Newline 253:254
RBrace 254:255
Newline 255:256
Newline 256:257
Ident 257:261 text="test"
Fn 262:264
Ident 265:287 text="map_get_absent_is_none"
LParen 287:288
RParen 288:289
LBrace 290:291
Newline 291:292
Let 296:299
Ident 300:301 text="m"
Assign 302:303
LBrace 304:305
Str 305:308 T("a")
Colon 308:309
Number 310:311 text="7" suf="" isf=0 int=7
RBrace 311:312
Newline 312:313
If 317:319
LParen 320:321
Let 321:324
Ident 325:329 text="some"
LParen 329:330
Ident 330:332 text="_v"
RParen 332:333
Assign 334:335
Ident 336:337 text="m"
Dot 337:338
Get 338:341
LParen 341:342
Str 342:351 T("missing")
RParen 351:352
RParen 352:353
LBrace 354:355
Newline 355:356
Ident 364:370 text="expect"
LParen 370:371
Str 371:383 T("unexpected")
RParen 383:384
Dot 384:385
Ident 385:389 text="toBe"
LParen 389:390
Str 390:451 T("map_get_absent_is_none: get returned some for an absent key")
RParen 451:452
Newline 452:453
RBrace 457:458
Else 459:463
LBrace 464:465
Newline 465:466
Ident 474:480 text="expect"
LParen 480:481
True 481:485
RParen 485:486
Dot 486:487
Ident 487:491 text="toBe"
LParen 491:492
True 492:496
RParen 496:497
Newline 497:498
RBrace 502:503
Newline 503:504
RBrace 504:505
Newline 505:506
Newline 506:507
Ident 507:511 text="test"
Fn 512:514
Ident 515:537 text="map_get_none_coalesces"
LParen 537:538
RParen 538:539
LBrace 540:541
Newline 541:542
Let 546:549
Ident 550:551 text="m"
Assign 552:553
LBrace 554:555
Str 555:558 T("x")
Colon 558:559
Number 560:562 text="42" suf="" isf=0 int=42
RBrace 562:563
Newline 563:564
Let 568:571
Ident 572:573 text="v"
Assign 574:575
Ident 576:577 text="m"
Dot 577:578
Get 578:581
LParen 581:582
Str 582:588 T("nope")
RParen 588:589
QuestionQuestion 590:592
Minus 593:594
Number 594:595 text="1" suf="" isf=0 int=1
Newline 595:596
Ident 600:606 text="expect"
LParen 606:607
Ident 607:608 text="v"
RParen 608:609
Dot 609:610
Ident 610:614 text="toBe"
LParen 614:615
Minus 615:616
Number 616:617 text="1" suf="" isf=0 int=1
RParen 617:618
Newline 618:619
Let 623:626
Ident 627:628 text="w"
Assign 629:630
Ident 631:632 text="m"
Dot 632:633
Get 633:636
LParen 636:637
Str 637:640 T("x")
RParen 640:641
QuestionQuestion 642:644
Minus 645:646
Number 646:647 text="1" suf="" isf=0 int=1
Newline 647:648
Ident 652:658 text="expect"
LParen 658:659
Ident 659:660 text="w"
RParen 660:661
Dot 661:662
Ident 662:666 text="toBe"
LParen 666:667
Number 667:669 text="42" suf="" isf=0 int=42
RParen 669:670
Newline 670:671
RBrace 671:672
Newline 672:673
Newline 673:674
Ident 674:678 text="test"
Fn 679:681
Ident 682:702 text="map_get_string_value"
LParen 702:703
RParen 703:704
LBrace 705:706
Newline 706:707
Let 711:714
Ident 715:716 text="m"
Assign 717:718
LBrace 719:720
Str 720:723 T("k")
Colon 723:724
Str 725:732 T("hello")
RBrace 732:733
Newline 733:734
Let 738:741
Ident 742:745 text="got"
Assign 746:747
Ident 748:749 text="m"
Dot 749:750
Get 750:753
LParen 753:754
Str 754:757 T("k")
RParen 757:758
Newline 758:759
If 763:765
LParen 766:767
Let 767:770
Ident 771:775 text="some"
LParen 775:776
Ident 776:777 text="v"
RParen 777:778
Assign 779:780
Ident 781:784 text="got"
RParen 784:785
LBrace 786:787
Newline 787:788
Ident 796:802 text="expect"
LParen 802:803
Ident 803:804 text="v"
RParen 804:805
Dot 805:806
Ident 806:810 text="toBe"
LParen 810:811
Str 811:818 T("hello")
RParen 818:819
Newline 819:820
RBrace 824:825
Else 826:830
LBrace 831:832
Newline 832:833
Ident 841:847 text="expect"
LParen 847:848
Str 848:860 T("unexpected")
RParen 860:861
Dot 861:862
Ident 862:866 text="toBe"
LParen 866:867
Str 867:926 T("map_get_string_value: get returned none for a present key")
RParen 926:927
Newline 927:928
RBrace 932:933
Newline 933:934
RBrace 934:935
Newline 935:936
Newline 936:937
Ident 937:941 text="test"
Fn 942:944
Ident 945:965 text="map_get_is_read_only"
LParen 965:966
RParen 966:967
LBrace 968:969
Newline 969:970
Let 974:977
Ident 978:979 text="m"
Assign 980:981
LBrace 982:983
Str 983:986 T("a")
Colon 986:987
Number 988:989 text="1" suf="" isf=0 int=1
Comma 989:990
Str 991:994 T("b")
Colon 994:995
Number 996:997 text="2" suf="" isf=0 int=2
RBrace 997:998
Newline 998:999
If 1003:1005
LParen 1006:1007
Let 1007:1010
Ident 1011:1015 text="some"
LParen 1015:1016
Ident 1016:1018 text="_v"
RParen 1018:1019
Assign 1020:1021
Ident 1022:1023 text="m"
Dot 1023:1024
Get 1024:1027
LParen 1027:1028
Str 1028:1031 T("a")
RParen 1031:1032
RParen 1032:1033
LBrace 1034:1035
Newline 1035:1036
Ident 1044:1050 text="expect"
LParen 1050:1051
True 1051:1055
RParen 1055:1056
Dot 1056:1057
Ident 1057:1061 text="toBe"
LParen 1061:1062
True 1062:1066
RParen 1066:1067
Newline 1067:1068
RBrace 1072:1073
Else 1074:1078
LBrace 1079:1080
Newline 1080:1081
Ident 1089:1095 text="expect"
LParen 1095:1096
Str 1096:1108 T("unexpected")
RParen 1108:1109
Dot 1109:1110
Ident 1110:1114 text="toBe"
LParen 1114:1115
Str 1115:1174 T("map_get_is_read_only: get returned none for a present key")
RParen 1174:1175
Newline 1175:1176
RBrace 1180:1181
Newline 1181:1182
Ident 1186:1192 text="expect"
LParen 1192:1193
Ident 1193:1194 text="m"
Dot 1194:1195
Ident 1195:1198 text="has"
LParen 1198:1199
Str 1199:1202 T("a")
RParen 1202:1203
RParen 1203:1204
Dot 1204:1205
Ident 1205:1209 text="toBe"
LParen 1209:1210
True 1210:1214
RParen 1214:1215
Newline 1215:1216
Ident 1220:1226 text="expect"
LParen 1226:1227
Ident 1227:1230 text="len"
LParen 1230:1231
Ident 1231:1232 text="m"
Dot 1232:1233
Ident 1233:1237 text="keys"
LParen 1237:1238
RParen 1238:1239
RParen 1239:1240
RParen 1240:1241
Dot 1241:1242
Ident 1242:1246 text="toBe"
LParen 1246:1247
Number 1247:1248 text="2" suf="" isf=0 int=2
RParen 1248:1249
Newline 1249:1250
RBrace 1250:1251
Newline 1251:1252
Newline 1252:1253
Ident 1253:1257 text="test"
Fn 1258:1260
Ident 1261:1283 text="map_get_tracks_removal"
LParen 1283:1284
RParen 1284:1285
LBrace 1286:1287
Newline 1287:1288
Let 1292:1295
Ident 1296:1297 text="m"
Assign 1298:1299
LBrace 1300:1301
Str 1301:1304 T("a")
Colon 1304:1305
Number 1306:1307 text="1" suf="" isf=0 int=1
RBrace 1307:1308
Newline 1308:1309
If 1313:1315
LParen 1316:1317
Let 1317:1320
Ident 1321:1325 text="some"
LParen 1325:1326
Ident 1326:1327 text="v"
RParen 1327:1328
Assign 1329:1330
Ident 1331:1332 text="m"
Dot 1332:1333
Get 1333:1336
LParen 1336:1337
Str 1337:1340 T("a")
RParen 1340:1341
RParen 1341:1342
LBrace 1343:1344
Newline 1344:1345
Ident 1353:1359 text="expect"
LParen 1359:1360
Ident 1360:1361 text="v"
RParen 1361:1362
Dot 1362:1363
Ident 1363:1367 text="toBe"
LParen 1367:1368
Number 1368:1369 text="1" suf="" isf=0 int=1
RParen 1369:1370
Newline 1370:1371
RBrace 1375:1376
Else 1377:1381
LBrace 1382:1383
Newline 1383:1384
Ident 1392:1398 text="expect"
LParen 1398:1399
Str 1399:1411 T("unexpected")
RParen 1411:1412
Dot 1412:1413
Ident 1413:1417 text="toBe"
LParen 1417:1418
Str 1418:1479 T("map_get_tracks_removal: get returned none for a present key")
RParen 1479:1480
Newline 1480:1481
RBrace 1485:1486
Newline 1486:1487
Ident 1491:1492 text="m"
Dot 1492:1493
Ident 1493:1499 text="remove"
LParen 1499:1500
Str 1500:1503 T("a")
RParen 1503:1504
Newline 1504:1505
If 1509:1511
LParen 1512:1513
Let 1513:1516
Ident 1517:1521 text="some"
LParen 1521:1522
Ident 1522:1524 text="_v"
RParen 1524:1525
Assign 1526:1527
Ident 1528:1529 text="m"
Dot 1529:1530
Get 1530:1533
LParen 1533:1534
Str 1534:1537 T("a")
RParen 1537:1538
RParen 1538:1539
LBrace 1540:1541
Newline 1541:1542
Ident 1550:1556 text="expect"
LParen 1556:1557
Str 1557:1569 T("unexpected")
RParen 1569:1570
Dot 1570:1571
Ident 1571:1575 text="toBe"
LParen 1575:1576
Str 1576:1632 T("map_get_tracks_removal: get returned some after remove")
RParen 1632:1633
Newline 1633:1634
RBrace 1638:1639
Else 1640:1644
LBrace 1645:1646
Newline 1646:1647
Ident 1655:1661 text="expect"
LParen 1661:1662
True 1662:1666
RParen 1666:1667
Dot 1667:1668
Ident 1668:1672 text="toBe"
LParen 1672:1673
True 1673:1677
RParen 1677:1678
Newline 1678:1679
RBrace 1683:1684
Newline 1684:1685
RBrace 1685:1686
Eof 1686:1686
