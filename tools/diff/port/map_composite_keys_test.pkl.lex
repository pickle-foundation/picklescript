Class 0:5
Ident 6:8 text="Pt"
LBrace 9:10
Newline 10:11
Ident 15:16 text="x"
Colon 16:17
Ident 18:21 text="int"
Newline 21:22
Ident 26:27 text="y"
Colon 27:28
Ident 29:32 text="int"
Newline 32:33
RBrace 33:34
Newline 34:35
Newline 35:36
Struct 36:42
Ident 43:47 text="Vec2"
LBrace 48:49
Newline 49:50
Ident 54:55 text="x"
Colon 55:56
Ident 57:60 text="int"
Newline 60:61
Ident 65:66 text="y"
Colon 66:67
Ident 68:71 text="int"
Newline 71:72
RBrace 72:73
Newline 73:74
Newline 74:75
Enum 75:79
Ident 80:85 text="Shape"
LBrace 86:87
Newline 87:88
Ident 92:98 text="Circle"
Newline 98:99
Ident 103:109 text="Square"
Newline 109:110
Ident 114:118 text="Rect"
LParen 118:119
Ident 119:120 text="w"
Colon 120:121
Ident 122:125 text="int"
Comma 125:126
Ident 127:128 text="h"
Colon 128:129
Ident 130:133 text="int"
RParen 133:134
Newline 134:135
RBrace 135:136
Newline 136:137
Newline 137:138
Ident 138:142 text="test"
Fn 143:145
Ident 146:173 text="list_keys_structural_lookup"
LParen 173:174
RParen 174:175
LBrace 176:177
Newline 177:178
Var 182:185
Ident 186:187 text="m"
Assign 188:189
LBrace 190:191
LBracket 191:192
Number 192:193 text="1" suf="" isf=0 int=1
Comma 193:194
Number 195:196 text="2" suf="" isf=0 int=2
RBracket 196:197
Colon 197:198
Str 199:205 T("pair")
RBrace 205:206
Newline 206:207
Ident 211:217 text="expect"
LParen 217:218
Ident 218:219 text="m"
LBracket 219:220
LBracket 220:221
Number 221:222 text="1" suf="" isf=0 int=1
Comma 222:223
Number 224:225 text="2" suf="" isf=0 int=2
RBracket 225:226
RBracket 226:227
RParen 227:228
Dot 228:229
Ident 229:233 text="toBe"
LParen 233:234
Str 234:240 T("pair")
RParen 240:241
Newline 241:242
Ident 246:252 text="expect"
LParen 252:253
Ident 253:254 text="m"
Dot 254:255
Ident 255:258 text="has"
LParen 258:259
LBracket 259:260
Number 260:261 text="1" suf="" isf=0 int=1
Comma 261:262
Number 263:264 text="2" suf="" isf=0 int=2
RBracket 264:265
RParen 265:266
RParen 266:267
Dot 267:268
Ident 268:272 text="toBe"
LParen 272:273
True 273:277
RParen 277:278
Newline 278:279
Ident 283:289 text="expect"
LParen 289:290
Ident 290:291 text="m"
Dot 291:292
Ident 292:295 text="has"
LParen 295:296
LBracket 296:297
Number 297:298 text="1" suf="" isf=0 int=1
Comma 298:299
Number 300:301 text="3" suf="" isf=0 int=3
RBracket 301:302
RParen 302:303
RParen 303:304
Dot 304:305
Ident 305:309 text="toBe"
LParen 309:310
False 310:315
RParen 315:316
Newline 316:317
Ident 321:322 text="m"
LBracket 322:323
LBracket 323:324
Number 324:325 text="3" suf="" isf=0 int=3
Comma 325:326
Number 327:328 text="4" suf="" isf=0 int=4
RBracket 328:329
RBracket 329:330
Assign 331:332
Str 333:340 T("other")
Newline 340:341
Ident 345:351 text="expect"
LParen 351:352
Ident 352:353 text="m"
LBracket 353:354
LBracket 354:355
Number 355:356 text="3" suf="" isf=0 int=3
Comma 356:357
Number 358:359 text="4" suf="" isf=0 int=4
RBracket 359:360
RBracket 360:361
RParen 361:362
Dot 362:363
Ident 363:367 text="toBe"
LParen 367:368
Str 368:375 T("other")
RParen 375:376
Newline 376:377
Ident 381:387 text="expect"
LParen 387:388
Ident 388:391 text="len"
LParen 391:392
Ident 392:393 text="m"
RParen 393:394
RParen 394:395
Dot 395:396
Ident 396:400 text="toBe"
LParen 400:401
Number 401:402 text="2" suf="" isf=0 int=2
RParen 402:403
Newline 403:404
Ident 408:409 text="m"
LBracket 409:410
LBracket 410:411
Number 411:412 text="1" suf="" isf=0 int=1
Comma 412:413
Number 414:415 text="2" suf="" isf=0 int=2
RBracket 415:416
RBracket 416:417
Assign 418:419
Str 420:430 T("replaced")
Newline 430:431
Ident 435:441 text="expect"
LParen 441:442
Ident 442:443 text="m"
LBracket 443:444
LBracket 444:445
Number 445:446 text="1" suf="" isf=0 int=1
Comma 446:447
Number 448:449 text="2" suf="" isf=0 int=2
RBracket 449:450
RBracket 450:451
RParen 451:452
Dot 452:453
Ident 453:457 text="toBe"
LParen 457:458
Str 458:468 T("replaced")
RParen 468:469
Newline 469:470
Ident 474:480 text="expect"
LParen 480:481
Ident 481:484 text="len"
LParen 484:485
Ident 485:486 text="m"
RParen 486:487
RParen 487:488
Dot 488:489
Ident 489:493 text="toBe"
LParen 493:494
Number 494:495 text="2" suf="" isf=0 int=2
RParen 495:496
Newline 496:497
Let 501:504
Ident 505:506 text="a"
Assign 507:508
Ident 509:510 text="m"
LBracket 510:511
LBracket 511:512
Number 512:513 text="3" suf="" isf=0 int=3
Comma 513:514
Number 515:516 text="4" suf="" isf=0 int=4
RBracket 516:517
RBracket 517:518
Newline 518:519
Ident 523:529 text="expect"
LParen 529:530
Ident 530:531 text="a"
RParen 531:532
Dot 532:533
Ident 533:537 text="toBe"
LParen 537:538
Str 538:545 T("other")
RParen 545:546
Newline 546:547
RBrace 547:548
Newline 548:549
Newline 549:550
Ident 550:554 text="test"
Fn 555:557
Ident 558:583 text="list_keys_order_sensitive"
LParen 583:584
RParen 584:585
LBrace 586:587
Newline 587:588
Var 592:595
Ident 596:597 text="m"
Assign 598:599
LBrace 600:601
LBracket 601:602
Number 602:603 text="1" suf="" isf=0 int=1
Comma 603:604
Number 605:606 text="2" suf="" isf=0 int=2
RBracket 606:607
Colon 607:608
Str 609:612 T("a")
Comma 612:613
LBracket 614:615
Number 615:616 text="2" suf="" isf=0 int=2
Comma 616:617
Number 618:619 text="1" suf="" isf=0 int=1
RBracket 619:620
Colon 620:621
Str 622:625 T("b")
RBrace 625:626
Newline 626:627
Ident 631:637 text="expect"
LParen 637:638
Ident 638:639 text="m"
LBracket 639:640
LBracket 640:641
Number 641:642 text="1" suf="" isf=0 int=1
Comma 642:643
Number 644:645 text="2" suf="" isf=0 int=2
RBracket 645:646
RBracket 646:647
RParen 647:648
Dot 648:649
Ident 649:653 text="toBe"
LParen 653:654
Str 654:657 T("a")
RParen 657:658
Newline 658:659
Ident 663:669 text="expect"
LParen 669:670
Ident 670:671 text="m"
LBracket 671:672
LBracket 672:673
Number 673:674 text="2" suf="" isf=0 int=2
Comma 674:675
Number 676:677 text="1" suf="" isf=0 int=1
RBracket 677:678
RBracket 678:679
RParen 679:680
Dot 680:681
Ident 681:685 text="toBe"
LParen 685:686
Str 686:689 T("b")
RParen 689:690
Newline 690:691
Ident 695:701 text="expect"
LParen 701:702
Ident 702:705 text="len"
LParen 705:706
Ident 706:707 text="m"
RParen 707:708
RParen 708:709
Dot 709:710
Ident 710:714 text="toBe"
LParen 714:715
Number 715:716 text="2" suf="" isf=0 int=2
RParen 716:717
Newline 717:718
RBrace 718:719
Newline 719:720
Newline 720:721
Ident 721:725 text="test"
Fn 726:728
Ident 729:759 text="map_keys_are_order_insensitive"
LParen 759:760
RParen 760:761
LBrace 762:763
Newline 763:764
Var 768:771
Ident 772:773 text="m"
Assign 774:775
LBrace 776:777
LBrace 777:778
Str 778:781 T("k")
Colon 781:782
Number 783:784 text="1" suf="" isf=0 int=1
RBrace 784:785
Colon 785:786
Str 787:795 T("nested")
RBrace 795:796
Newline 796:797
Ident 801:807 text="expect"
LParen 807:808
Ident 808:809 text="m"
LBracket 809:810
LBrace 810:811
Str 811:814 T("k")
Colon 814:815
Number 816:817 text="1" suf="" isf=0 int=1
RBrace 817:818
RBracket 818:819
RParen 819:820
Dot 820:821
Ident 821:825 text="toBe"
LParen 825:826
Str 826:834 T("nested")
RParen 834:835
Newline 835:836
Ident 840:841 text="m"
LBracket 841:842
LBrace 842:843
Str 843:846 T("k")
Colon 846:847
Number 848:849 text="2" suf="" isf=0 int=2
RBrace 849:850
RBracket 850:851
Assign 852:853
Str 854:865 T("different")
Newline 865:866
Ident 870:876 text="expect"
LParen 876:877
Ident 877:878 text="m"
LBracket 878:879
LBrace 879:880
Str 880:883 T("k")
Colon 883:884
Number 885:886 text="2" suf="" isf=0 int=2
RBrace 886:887
RBracket 887:888
RParen 888:889
Dot 889:890
Ident 890:894 text="toBe"
LParen 894:895
Str 895:906 T("different")
RParen 906:907
Newline 907:908
Ident 912:918 text="expect"
LParen 918:919
Ident 919:920 text="m"
LBracket 920:921
LBrace 921:922
Str 922:925 T("k")
Colon 925:926
Number 927:928 text="1" suf="" isf=0 int=1
RBrace 928:929
RBracket 929:930
RParen 930:931
Dot 931:932
Ident 932:936 text="toBe"
LParen 936:937
Str 937:945 T("nested")
RParen 945:946
Newline 946:947
Ident 951:957 text="expect"
LParen 957:958
Ident 958:961 text="len"
LParen 961:962
Ident 962:963 text="m"
RParen 963:964
RParen 964:965
Dot 965:966
Ident 966:970 text="toBe"
LParen 970:971
Number 971:972 text="2" suf="" isf=0 int=2
RParen 972:973
Newline 973:974
RBrace 974:975
Newline 975:976
Newline 976:977
Ident 977:981 text="test"
Fn 982:984
Ident 985:1006 text="class_and_struct_keys"
LParen 1006:1007
RParen 1007:1008
LBrace 1009:1010
Newline 1010:1011
Let 1015:1018
Ident 1019:1020 text="p"
Assign 1021:1022
Ident 1023:1025 text="Pt"
LParen 1025:1026
Number 1026:1027 text="5" suf="" isf=0 int=5
Comma 1027:1028
Number 1029:1030 text="6" suf="" isf=0 int=6
RParen 1030:1031
Newline 1031:1032
Let 1036:1039
Ident 1040:1042 text="p2"
Assign 1043:1044
Ident 1045:1047 text="Pt"
LParen 1047:1048
Number 1048:1049 text="5" suf="" isf=0 int=5
Comma 1049:1050
Number 1051:1052 text="6" suf="" isf=0 int=6
RParen 1052:1053
Newline 1053:1054
Var 1058:1061
Ident 1062:1067 text="byobj"
Assign 1068:1069
LBrace 1070:1071
Ident 1071:1072 text="p"
Colon 1072:1073
Number 1074:1076 text="10" suf="" isf=0 int=10
RBrace 1076:1077
Newline 1077:1078
Ident 1082:1088 text="expect"
LParen 1088:1089
Ident 1089:1094 text="byobj"
LBracket 1094:1095
Ident 1095:1097 text="p2"
RBracket 1097:1098
RParen 1098:1099
Dot 1099:1100
Ident 1100:1104 text="toBe"
LParen 1104:1105
Number 1105:1107 text="10" suf="" isf=0 int=10
RParen 1107:1108
Newline 1108:1109
Ident 1113:1118 text="byobj"
LBracket 1118:1119
Ident 1119:1121 text="Pt"
LParen 1121:1122
Number 1122:1123 text="7" suf="" isf=0 int=7
Comma 1123:1124
Number 1125:1126 text="8" suf="" isf=0 int=8
RParen 1126:1127
RBracket 1127:1128
Assign 1129:1130
Number 1131:1133 text="20" suf="" isf=0 int=20
Newline 1133:1134
Ident 1138:1144 text="expect"
LParen 1144:1145
Ident 1145:1150 text="byobj"
LBracket 1150:1151
Ident 1151:1153 text="Pt"
LParen 1153:1154
Number 1154:1155 text="7" suf="" isf=0 int=7
Comma 1155:1156
Number 1157:1158 text="8" suf="" isf=0 int=8
RParen 1158:1159
RBracket 1159:1160
RParen 1160:1161
Dot 1161:1162
Ident 1162:1166 text="toBe"
LParen 1166:1167
Number 1167:1169 text="20" suf="" isf=0 int=20
RParen 1169:1170
Newline 1170:1171
Ident 1175:1181 text="expect"
LParen 1181:1182
Ident 1182:1185 text="len"
LParen 1185:1186
Ident 1186:1191 text="byobj"
RParen 1191:1192
RParen 1192:1193
Dot 1193:1194
Ident 1194:1198 text="toBe"
LParen 1198:1199
Number 1199:1200 text="2" suf="" isf=0 int=2
RParen 1200:1201
Newline 1201:1202
Let 1206:1209
Ident 1210:1211 text="v"
Assign 1212:1213
Ident 1214:1218 text="Vec2"
LParen 1218:1219
Number 1219:1220 text="1" suf="" isf=0 int=1
Comma 1220:1221
Number 1222:1223 text="2" suf="" isf=0 int=2
RParen 1223:1224
Newline 1224:1225
Var 1229:1232
Ident 1233:1238 text="byvec"
Assign 1239:1240
LBrace 1241:1242
Ident 1242:1243 text="v"
Colon 1243:1244
Str 1245:1248 T("v")
RBrace 1248:1249
Newline 1249:1250
Ident 1254:1260 text="expect"
LParen 1260:1261
Ident 1261:1266 text="byvec"
LBracket 1266:1267
Ident 1267:1271 text="Vec2"
LParen 1271:1272
Number 1272:1273 text="1" suf="" isf=0 int=1
Comma 1273:1274
Number 1275:1276 text="2" suf="" isf=0 int=2
RParen 1276:1277
RBracket 1277:1278
RParen 1278:1279
Dot 1279:1280
Ident 1280:1284 text="toBe"
LParen 1284:1285
Str 1285:1288 T("v")
RParen 1288:1289
Newline 1289:1290
Ident 1294:1300 text="expect"
LParen 1300:1301
Ident 1301:1306 text="byvec"
LBracket 1306:1307
Ident 1307:1311 text="Vec2"
LParen 1311:1312
Number 1312:1313 text="9" suf="" isf=0 int=9
Comma 1313:1314
Number 1315:1316 text="9" suf="" isf=0 int=9
RParen 1316:1317
RBracket 1317:1318
RParen 1318:1319
Dot 1319:1320
Ident 1320:1324 text="toBe"
LParen 1324:1325
Str 1325:1327
RParen 1327:1328
Newline 1328:1329
RBrace 1329:1330
Newline 1330:1331
Newline 1331:1332
Ident 1332:1336 text="test"
Fn 1337:1339
Ident 1340:1349 text="enum_keys"
LParen 1349:1350
RParen 1350:1351
LBrace 1352:1353
Newline 1353:1354
Var 1358:1361
Ident 1362:1364 text="es"
Assign 1365:1366
LBrace 1367:1368
Ident 1368:1373 text="Shape"
Dot 1373:1374
Ident 1374:1380 text="Circle"
Colon 1380:1381
Str 1382:1385 T("c")
Comma 1385:1386
Ident 1387:1392 text="Shape"
Dot 1392:1393
Ident 1393:1397 text="Rect"
LParen 1397:1398
Number 1398:1399 text="2" suf="" isf=0 int=2
Comma 1399:1400
Number 1401:1402 text="3" suf="" isf=0 int=3
RParen 1402:1403
Colon 1403:1404
Str 1405:1408 T("r")
RBrace 1408:1409
Newline 1409:1410
Ident 1414:1420 text="expect"
LParen 1420:1421
Ident 1421:1423 text="es"
LBracket 1423:1424
Ident 1424:1429 text="Shape"
Dot 1429:1430
Ident 1430:1436 text="Circle"
RBracket 1436:1437
RParen 1437:1438
Dot 1438:1439
Ident 1439:1443 text="toBe"
LParen 1443:1444
Str 1444:1447 T("c")
RParen 1447:1448
Newline 1448:1449
Ident 1453:1459 text="expect"
LParen 1459:1460
Ident 1460:1462 text="es"
LBracket 1462:1463
Ident 1463:1468 text="Shape"
Dot 1468:1469
Ident 1469:1475 text="Square"
RBracket 1475:1476
RParen 1476:1477
Dot 1477:1478
Ident 1478:1482 text="toBe"
LParen 1482:1483
Str 1483:1485
RParen 1485:1486
Newline 1486:1487
Ident 1491:1497 text="expect"
LParen 1497:1498
Ident 1498:1500 text="es"
LBracket 1500:1501
Ident 1501:1506 text="Shape"
Dot 1506:1507
Ident 1507:1511 text="Rect"
LParen 1511:1512
Number 1512:1513 text="2" suf="" isf=0 int=2
Comma 1513:1514
Number 1515:1516 text="3" suf="" isf=0 int=3
RParen 1516:1517
RBracket 1517:1518
RParen 1518:1519
Dot 1519:1520
Ident 1520:1524 text="toBe"
LParen 1524:1525
Str 1525:1528 T("r")
RParen 1528:1529
Newline 1529:1530
Ident 1534:1540 text="expect"
LParen 1540:1541
Ident 1541:1543 text="es"
Dot 1543:1544
Ident 1544:1547 text="has"
LParen 1547:1548
Ident 1548:1553 text="Shape"
Dot 1553:1554
Ident 1554:1558 text="Rect"
LParen 1558:1559
Number 1559:1560 text="2" suf="" isf=0 int=2
Comma 1560:1561
Number 1562:1563 text="3" suf="" isf=0 int=3
RParen 1563:1564
RParen 1564:1565
RParen 1565:1566
Dot 1566:1567
Ident 1567:1571 text="toBe"
LParen 1571:1572
True 1572:1576
RParen 1576:1577
Newline 1577:1578
Ident 1582:1588 text="expect"
LParen 1588:1589
Ident 1589:1591 text="es"
Dot 1591:1592
Ident 1592:1595 text="has"
LParen 1595:1596
Ident 1596:1601 text="Shape"
Dot 1601:1602
Ident 1602:1606 text="Rect"
LParen 1606:1607
Number 1607:1608 text="3" suf="" isf=0 int=3
Comma 1608:1609
Number 1610:1611 text="2" suf="" isf=0 int=2
RParen 1611:1612
RParen 1612:1613
RParen 1613:1614
Dot 1614:1615
Ident 1615:1619 text="toBe"
LParen 1619:1620
False 1620:1625
RParen 1625:1626
Newline 1626:1627
RBrace 1627:1628
Newline 1628:1629
Newline 1629:1630
Ident 1630:1634 text="test"
Fn 1635:1637
Ident 1638:1673 text="class_and_list_keys_are_independent"
LParen 1673:1674
RParen 1674:1675
LBrace 1676:1677
Newline 1677:1678
Var 1682:1685
Ident 1686:1688 text="ml"
Assign 1689:1690
LBrace 1691:1692
LBracket 1692:1693
Number 1693:1695 text="10" suf="" isf=0 int=10
Comma 1695:1696
Number 1697:1699 text="20" suf="" isf=0 int=20
RBracket 1699:1700
Colon 1700:1701
Str 1702:1708 T("list")
RBrace 1708:1709
Newline 1709:1710
Var 1714:1717
Ident 1718:1720 text="mc"
Assign 1721:1722
LBrace 1723:1724
Ident 1724:1726 text="Pt"
LParen 1726:1727
Number 1727:1729 text="10" suf="" isf=0 int=10
Comma 1729:1730
Number 1731:1733 text="20" suf="" isf=0 int=20
RParen 1733:1734
Colon 1734:1735
Str 1736:1743 T("class")
RBrace 1743:1744
Newline 1744:1745
Ident 1749:1755 text="expect"
LParen 1755:1756
Ident 1756:1758 text="ml"
LBracket 1758:1759
LBracket 1759:1760
Number 1760:1762 text="10" suf="" isf=0 int=10
Comma 1762:1763
Number 1764:1766 text="20" suf="" isf=0 int=20
RBracket 1766:1767
RBracket 1767:1768
RParen 1768:1769
Dot 1769:1770
Ident 1770:1774 text="toBe"
LParen 1774:1775
Str 1775:1781 T("list")
RParen 1781:1782
Newline 1782:1783
Ident 1787:1793 text="expect"
LParen 1793:1794
Ident 1794:1796 text="mc"
LBracket 1796:1797
Ident 1797:1799 text="Pt"
LParen 1799:1800
Number 1800:1802 text="10" suf="" isf=0 int=10
Comma 1802:1803
Number 1804:1806 text="20" suf="" isf=0 int=20
RParen 1806:1807
RBracket 1807:1808
RParen 1808:1809
Dot 1809:1810
Ident 1810:1814 text="toBe"
LParen 1814:1815
Str 1815:1822 T("class")
RParen 1822:1823
Newline 1823:1824
Ident 1828:1834 text="expect"
LParen 1834:1835
Ident 1835:1838 text="len"
LParen 1838:1839
Ident 1839:1841 text="ml"
RParen 1841:1842
RParen 1842:1843
Dot 1843:1844
Ident 1844:1848 text="toBe"
LParen 1848:1849
Number 1849:1850 text="1" suf="" isf=0 int=1
RParen 1850:1851
Newline 1851:1852
Ident 1856:1862 text="expect"
LParen 1862:1863
Ident 1863:1866 text="len"
LParen 1866:1867
Ident 1867:1869 text="mc"
RParen 1869:1870
RParen 1870:1871
Dot 1871:1872
Ident 1872:1876 text="toBe"
LParen 1876:1877
Number 1877:1878 text="1" suf="" isf=0 int=1
RParen 1878:1879
Newline 1879:1880
RBrace 1880:1881
Newline 1881:1882
Newline 1882:1883
Ident 1883:1887 text="test"
Fn 1888:1890
Ident 1891:1921 text="composite_keys_grow_and_rehash"
LParen 1921:1922
RParen 1922:1923
LBrace 1924:1925
Newline 1925:1926
Var 1930:1933
Ident 1934:1935 text="m"
Assign 1936:1937
LBrace 1938:1939
LBracket 1939:1940
Number 1940:1941 text="0" suf="" isf=0 int=0
Comma 1941:1942
Number 1943:1944 text="0" suf="" isf=0 int=0
RBracket 1944:1945
Colon 1945:1946
Str 1947:1953 T("seed")
RBrace 1953:1954
Newline 1954:1955
For 1959:1962
LParen 1963:1964
Ident 1964:1965 text="i"
In 1966:1968
Number 1969:1970 text="1" suf="" isf=0 int=1
Range 1970:1972
Number 1972:1975 text="150" suf="" isf=0 int=150
RParen 1975:1976
LBrace 1977:1978
Newline 1978:1979
Ident 1987:1988 text="m"
LBracket 1988:1989
LBracket 1989:1990
Ident 1990:1991 text="i"
Comma 1991:1992
Ident 1993:1994 text="i"
Star 1995:1996
Number 1997:1998 text="2" suf="" isf=0 int=2
RBracket 1998:1999
RBracket 1999:2000
Assign 2001:2002
Str 2003:2008 E[Ident 2005:2006 text="i"; Eof 2007:2007]
Newline 2008:2009
RBrace 2013:2014
Newline 2014:2015
Ident 2019:2025 text="expect"
LParen 2025:2026
Ident 2026:2029 text="len"
LParen 2029:2030
Ident 2030:2031 text="m"
RParen 2031:2032
RParen 2032:2033
Dot 2033:2034
Ident 2034:2038 text="toBe"
LParen 2038:2039
Number 2039:2042 text="150" suf="" isf=0 int=150
RParen 2042:2043
Newline 2043:2044
Ident 2048:2054 text="expect"
LParen 2054:2055
Ident 2055:2056 text="m"
LBracket 2056:2057
LBracket 2057:2058
Number 2058:2059 text="0" suf="" isf=0 int=0
Comma 2059:2060
Number 2061:2062 text="0" suf="" isf=0 int=0
RBracket 2062:2063
RBracket 2063:2064
RParen 2064:2065
Dot 2065:2066
Ident 2066:2070 text="toBe"
LParen 2070:2071
Str 2071:2077 T("seed")
RParen 2077:2078
Newline 2078:2079
Ident 2083:2089 text="expect"
LParen 2089:2090
Ident 2090:2091 text="m"
LBracket 2091:2092
LBracket 2092:2093
Number 2093:2095 text="75" suf="" isf=0 int=75
Comma 2095:2096
Number 2097:2100 text="150" suf="" isf=0 int=150
RBracket 2100:2101
RBracket 2101:2102
RParen 2102:2103
Dot 2103:2104
Ident 2104:2108 text="toBe"
LParen 2108:2109
Str 2109:2113 T("75")
RParen 2113:2114
Newline 2114:2115
Var 2119:2122
Ident 2123:2124 text="n"
Assign 2125:2126
Number 2127:2128 text="0" suf="" isf=0 int=0
Newline 2128:2129
For 2133:2136
LParen 2137:2138
LParen 2138:2139
Ident 2139:2140 text="k"
Comma 2140:2141
Ident 2142:2143 text="v"
RParen 2143:2144
In 2145:2147
Ident 2148:2149 text="m"
RParen 2149:2150
LBrace 2151:2152
Newline 2152:2153
If 2161:2163
LParen 2164:2165
Ident 2165:2166 text="m"
LBracket 2166:2167
Ident 2167:2168 text="k"
RBracket 2168:2169
EqEq 2170:2172
Ident 2173:2174 text="v"
RParen 2174:2175
LBrace 2176:2177
Newline 2177:2178
Ident 2190:2191 text="n"
PlusEq 2192:2194
Number 2195:2196 text="1" suf="" isf=0 int=1
Newline 2196:2197
RBrace 2205:2206
Newline 2206:2207
RBrace 2211:2212
Newline 2212:2213
Ident 2217:2223 text="expect"
LParen 2223:2224
Ident 2224:2225 text="n"
RParen 2225:2226
Dot 2226:2227
Ident 2227:2231 text="toBe"
LParen 2231:2232
Number 2232:2235 text="150" suf="" isf=0 int=150
RParen 2235:2236
Newline 2236:2237
RBrace 2237:2238
Newline 2238:2239
Newline 2239:2240
Ident 2240:2244 text="test"
Fn 2245:2247
Ident 2248:2264 text="composite_remove"
LParen 2264:2265
RParen 2265:2266
LBrace 2267:2268
Newline 2268:2269
Var 2273:2276
Ident 2277:2278 text="m"
Assign 2279:2280
LBrace 2281:2282
LBracket 2282:2283
Number 2283:2284 text="1" suf="" isf=0 int=1
Comma 2284:2285
Number 2286:2287 text="1" suf="" isf=0 int=1
RBracket 2287:2288
Colon 2288:2289
Str 2290:2295 T("one")
Comma 2295:2296
LBracket 2297:2298
Number 2298:2299 text="2" suf="" isf=0 int=2
Comma 2299:2300
Number 2301:2302 text="2" suf="" isf=0 int=2
RBracket 2302:2303
Colon 2303:2304
Str 2305:2310 T("two")
RBrace 2310:2311
Newline 2311:2312
If 2316:2318
LParen 2319:2320
Let 2320:2323
Ident 2324:2328 text="some"
LParen 2328:2329
Ident 2329:2330 text="r"
RParen 2330:2331
Assign 2332:2333
Ident 2334:2335 text="m"
Dot 2335:2336
Ident 2336:2342 text="remove"
LParen 2342:2343
LBracket 2343:2344
Number 2344:2345 text="1" suf="" isf=0 int=1
Comma 2345:2346
Number 2347:2348 text="1" suf="" isf=0 int=1
RBracket 2348:2349
RParen 2349:2350
RParen 2350:2351
LBrace 2352:2353
Newline 2353:2354
Ident 2362:2368 text="expect"
LParen 2368:2369
Ident 2369:2370 text="r"
RParen 2370:2371
Dot 2371:2372
Ident 2372:2376 text="toBe"
LParen 2376:2377
Str 2377:2382 T("one")
RParen 2382:2383
Newline 2383:2384
RBrace 2388:2389
Else 2390:2394
LBrace 2395:2396
Newline 2396:2397
Ident 2405:2411 text="expect"
LParen 2411:2412
Str 2412:2424 T("unexpected")
RParen 2424:2425
Dot 2425:2426
Ident 2426:2430 text="toBe"
LParen 2430:2431
Str 2431:2489 T("composite_remove: remove returned none for a present key")
RParen 2489:2490
Newline 2490:2491
RBrace 2495:2496
Newline 2496:2497
Ident 2501:2507 text="expect"
LParen 2507:2508
Ident 2508:2509 text="m"
Dot 2509:2510
Ident 2510:2513 text="has"
LParen 2513:2514
LBracket 2514:2515
Number 2515:2516 text="1" suf="" isf=0 int=1
Comma 2516:2517
Number 2518:2519 text="1" suf="" isf=0 int=1
RBracket 2519:2520
RParen 2520:2521
RParen 2521:2522
Dot 2522:2523
Ident 2523:2527 text="toBe"
LParen 2527:2528
False 2528:2533
RParen 2533:2534
Newline 2534:2535
Ident 2539:2545 text="expect"
LParen 2545:2546
Ident 2546:2547 text="m"
Dot 2547:2548
Ident 2548:2551 text="has"
LParen 2551:2552
LBracket 2552:2553
Number 2553:2554 text="2" suf="" isf=0 int=2
Comma 2554:2555
Number 2556:2557 text="2" suf="" isf=0 int=2
RBracket 2557:2558
RParen 2558:2559
RParen 2559:2560
Dot 2560:2561
Ident 2561:2565 text="toBe"
LParen 2565:2566
True 2566:2570
RParen 2570:2571
Newline 2571:2572
Ident 2576:2582 text="expect"
LParen 2582:2583
Ident 2583:2586 text="len"
LParen 2586:2587
Ident 2587:2588 text="m"
RParen 2588:2589
RParen 2589:2590
Dot 2590:2591
Ident 2591:2595 text="toBe"
LParen 2595:2596
Number 2596:2597 text="1" suf="" isf=0 int=1
RParen 2597:2598
Newline 2598:2599
Ident 2603:2609 text="expect"
LParen 2609:2610
Ident 2610:2611 text="m"
Dot 2611:2612
Ident 2612:2618 text="remove"
LParen 2618:2619
LBracket 2619:2620
Number 2620:2621 text="9" suf="" isf=0 int=9
Comma 2621:2622
Number 2623:2624 text="9" suf="" isf=0 int=9
RBracket 2624:2625
RParen 2625:2626
RParen 2626:2627
Dot 2627:2628
Ident 2628:2632 text="toBe"
LParen 2632:2633
None 2633:2637
RParen 2637:2638
Newline 2638:2639
RBrace 2639:2640
Newline 2640:2641
Newline 2641:2642
Ident 2642:2646 text="test"
Fn 2647:2649
Ident 2650:2681 text="for_in_over_composite_keyed_map"
LParen 2681:2682
RParen 2682:2683
LBrace 2684:2685
Newline 2685:2686
Var 2690:2693
Ident 2694:2697 text="ixs"
Assign 2698:2699
LBrace 2700:2701
LBracket 2701:2702
Number 2702:2704 text="10" suf="" isf=0 int=10
Comma 2704:2705
Number 2706:2708 text="20" suf="" isf=0 int=20
RBracket 2708:2709
Colon 2709:2710
LBracket 2711:2712
Number 2712:2713 text="1" suf="" isf=0 int=1
Comma 2713:2714
Number 2715:2716 text="2" suf="" isf=0 int=2
Comma 2716:2717
Number 2718:2719 text="3" suf="" isf=0 int=3
RBracket 2719:2720
Comma 2720:2721
LBracket 2722:2723
Number 2723:2725 text="30" suf="" isf=0 int=30
Comma 2725:2726
Number 2727:2729 text="40" suf="" isf=0 int=40
RBracket 2729:2730
Colon 2730:2731
LBracket 2732:2733
Number 2733:2734 text="4" suf="" isf=0 int=4
Comma 2734:2735
Number 2736:2737 text="5" suf="" isf=0 int=5
RBracket 2737:2738
RBrace 2738:2739
Newline 2739:2740
Var 2744:2747
Ident 2748:2754 text="sum_k0"
Assign 2755:2756
Number 2757:2758 text="0" suf="" isf=0 int=0
Newline 2758:2759
Var 2763:2766
Ident 2767:2772 text="sum_v"
Assign 2773:2774
Number 2775:2776 text="0" suf="" isf=0 int=0
Newline 2776:2777
Var 2781:2784
Ident 2785:2786 text="n"
Assign 2787:2788
Number 2789:2790 text="0" suf="" isf=0 int=0
Newline 2790:2791
For 2795:2798
LParen 2799:2800
LParen 2800:2801
Ident 2801:2802 text="k"
Comma 2802:2803
Ident 2804:2805 text="v"
RParen 2805:2806
In 2807:2809
Ident 2810:2813 text="ixs"
RParen 2813:2814
LBrace 2815:2816
Newline 2816:2817
Ident 2825:2826 text="n"
PlusEq 2827:2829
Number 2830:2831 text="1" suf="" isf=0 int=1
Newline 2831:2832
Ident 2840:2846 text="sum_k0"
PlusEq 2847:2849
Ident 2850:2851 text="k"
LBracket 2851:2852
Number 2852:2853 text="0" suf="" isf=0 int=0
RBracket 2853:2854
Newline 2854:2855
Ident 2863:2868 text="sum_v"
PlusEq 2869:2871
Ident 2872:2875 text="len"
LParen 2875:2876
Ident 2876:2877 text="v"
RParen 2877:2878
Newline 2878:2879
RBrace 2883:2884
Newline 2884:2885
Ident 2889:2895 text="expect"
LParen 2895:2896
Ident 2896:2897 text="n"
RParen 2897:2898
Dot 2898:2899
Ident 2899:2903 text="toBe"
LParen 2903:2904
Number 2904:2905 text="2" suf="" isf=0 int=2
RParen 2905:2906
Newline 2906:2907
Ident 2911:2917 text="expect"
LParen 2917:2918
Ident 2918:2924 text="sum_k0"
RParen 2924:2925
Dot 2925:2926
Ident 2926:2930 text="toBe"
LParen 2930:2931
Number 2931:2933 text="40" suf="" isf=0 int=40
RParen 2933:2934
Newline 2934:2935
Ident 2939:2945 text="expect"
LParen 2945:2946
Ident 2946:2951 text="sum_v"
RParen 2951:2952
Dot 2952:2953
Ident 2953:2957 text="toBe"
LParen 2957:2958
Number 2958:2959 text="5" suf="" isf=0 int=5
RParen 2959:2960
Newline 2960:2961
RBrace 2961:2962
Eof 2962:2962
