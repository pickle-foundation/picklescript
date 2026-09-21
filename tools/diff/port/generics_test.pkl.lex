Fn 0:2
Ident 3:8 text="id_fn"
Lt 8:9
Ident 9:10 text="T"
Gt 10:11
LParen 11:12
Ident 12:13 text="x"
Colon 13:14
Ident 15:16 text="T"
RParen 16:17
Arrow 18:20
Ident 21:22 text="T"
LBrace 23:24
Newline 24:25
Ident 29:30 text="x"
Newline 30:31
RBrace 31:32
Newline 32:33
Newline 33:34
Fn 34:36
Ident 37:42 text="twice"
Lt 42:43
Ident 43:44 text="T"
Gt 44:45
LParen 45:46
Ident 46:47 text="x"
Colon 47:48
Ident 49:50 text="T"
RParen 50:51
Arrow 52:54
Ident 55:56 text="T"
LBrace 57:58
Newline 58:59
Ident 63:68 text="id_fn"
Lt 68:69
Ident 69:70 text="T"
Gt 70:71
LParen 71:72
Ident 72:77 text="id_fn"
Lt 77:78
Ident 78:79 text="T"
Gt 79:80
LParen 80:81
Ident 81:82 text="x"
RParen 82:83
RParen 83:84
Newline 84:85
RBrace 85:86
Newline 86:87
Newline 87:88
Fn 88:90
Ident 91:96 text="first"
Lt 96:97
Ident 97:98 text="T"
Gt 98:99
LParen 99:100
Ident 100:102 text="xs"
Colon 102:103
Ident 104:108 text="List"
Lt 108:109
Ident 109:110 text="T"
Gt 110:111
Comma 111:112
Ident 113:121 text="fallback"
Colon 121:122
Ident 123:124 text="T"
RParen 124:125
Arrow 126:128
Ident 129:130 text="T"
LBrace 131:132
Newline 132:133
If 137:139
LParen 140:141
Ident 141:144 text="len"
LParen 144:145
Ident 145:147 text="xs"
RParen 147:148
Gt 149:150
Number 151:152 text="0" suf="" isf=0 int=0
RParen 152:153
LBrace 154:155
Newline 155:156
Ident 164:166 text="xs"
LBracket 166:167
Number 167:168 text="0" suf="" isf=0 int=0
RBracket 168:169
Newline 169:170
RBrace 174:175
Else 176:180
LBrace 181:182
Newline 182:183
Ident 191:199 text="fallback"
Newline 199:200
RBrace 204:205
Newline 205:206
RBrace 206:207
Newline 207:208
Newline 208:209
Fn 209:211
Ident 212:218 text="choose"
Lt 218:219
Ident 219:220 text="A"
Comma 220:221
Ident 222:223 text="B"
Gt 223:224
LParen 224:225
Ident 225:232 text="primary"
Colon 232:233
Ident 234:235 text="A"
Comma 235:236
Ident 237:246 text="secondary"
Colon 246:247
Ident 248:249 text="B"
RParen 249:250
Arrow 251:253
Ident 254:255 text="B"
LBrace 256:257
Newline 257:258
Ident 262:271 text="secondary"
Newline 271:272
RBrace 272:273
Newline 273:274
Newline 274:275
Fn 275:277
Ident 278:289 text="apply_twice"
Lt 289:290
Ident 290:291 text="T"
Gt 291:292
LParen 292:293
Ident 293:294 text="f"
Colon 294:295
Fn 296:298
LParen 299:300
Ident 300:301 text="T"
RParen 301:302
Arrow 303:305
Ident 306:307 text="T"
Comma 307:308
Ident 309:310 text="x"
Colon 310:311
Ident 312:313 text="T"
RParen 313:314
Arrow 315:317
Ident 318:319 text="T"
LBrace 320:321
Newline 321:322
Ident 326:327 text="f"
LParen 327:328
Ident 328:329 text="f"
LParen 329:330
Ident 330:331 text="x"
RParen 331:332
RParen 332:333
Newline 333:334
RBrace 334:335
Newline 335:336
Newline 336:337
Fn 337:339
Ident 340:348 text="pick_one"
Lt 348:349
Ident 349:350 text="T"
Gt 350:351
LParen 351:352
Ident 352:354 text="xs"
Colon 354:355
Ident 356:360 text="List"
Lt 360:361
Ident 361:362 text="T"
Gt 362:363
Comma 363:364
Ident 365:373 text="fallback"
Colon 373:374
Ident 375:376 text="T"
Question 376:377
RParen 377:378
Arrow 379:381
Ident 382:383 text="T"
LBrace 384:385
Newline 385:386
If 390:392
LParen 393:394
Ident 394:397 text="len"
LParen 397:398
Ident 398:400 text="xs"
RParen 400:401
Gt 402:403
Number 404:405 text="0" suf="" isf=0 int=0
RParen 405:406
LBrace 407:408
Newline 408:409
Ident 417:419 text="xs"
LBracket 419:420
Number 420:421 text="0" suf="" isf=0 int=0
RBracket 421:422
Newline 422:423
RBrace 427:428
Else 429:433
LBrace 434:435
Newline 435:436
Ident 444:452 text="fallback"
Question 452:453
Newline 453:454
RBrace 458:459
Newline 459:460
RBrace 460:461
Newline 461:462
Newline 462:463
Fn 463:465
Ident 466:472 text="negate"
LParen 472:473
Ident 473:474 text="x"
Colon 474:475
Ident 476:479 text="int"
RParen 479:480
Arrow 481:483
Ident 484:487 text="int"
LBrace 488:489
Newline 489:490
Minus 494:495
Ident 495:496 text="x"
Newline 496:497
RBrace 497:498
Newline 498:499
Newline 499:500
Ident 500:504 text="test"
Fn 505:507
Ident 508:523 text="generic_id_ints"
LParen 523:524
RParen 524:525
LBrace 526:527
Newline 527:528
Ident 532:538 text="expect"
LParen 538:539
Ident 539:544 text="id_fn"
Lt 544:545
Ident 545:548 text="int"
Gt 548:549
LParen 549:550
Number 550:551 text="3" suf="" isf=0 int=3
RParen 551:552
RParen 552:553
Dot 553:554
Ident 554:558 text="toBe"
LParen 558:559
Number 559:560 text="3" suf="" isf=0 int=3
RParen 560:561
Newline 561:562
Ident 566:572 text="expect"
LParen 572:573
Ident 573:578 text="id_fn"
Lt 578:579
Ident 579:582 text="int"
Gt 582:583
LParen 583:584
Minus 584:585
Number 585:586 text="7" suf="" isf=0 int=7
RParen 586:587
RParen 587:588
Dot 588:589
Ident 589:593 text="toBe"
LParen 593:594
Minus 594:595
Number 595:596 text="7" suf="" isf=0 int=7
RParen 596:597
Newline 597:598
RBrace 598:599
Newline 599:600
Newline 600:601
Ident 601:605 text="test"
Fn 606:608
Ident 609:627 text="generic_id_strings"
LParen 627:628
RParen 628:629
LBrace 630:631
Newline 631:632
Ident 636:642 text="expect"
LParen 642:643
Ident 643:648 text="id_fn"
Lt 648:649
Ident 649:655 text="string"
Gt 655:656
LParen 656:657
Str 657:664 T("hello")
RParen 664:665
RParen 665:666
Dot 666:667
Ident 667:671 text="toBe"
LParen 671:672
Str 672:679 T("hello")
RParen 679:680
Newline 680:681
RBrace 681:682
Newline 682:683
Newline 683:684
Ident 684:688 text="test"
Fn 689:691
Ident 692:709 text="generic_id_floats"
LParen 709:710
RParen 710:711
LBrace 712:713
Newline 713:714
Ident 718:724 text="expect"
LParen 724:725
Ident 725:730 text="id_fn"
Lt 730:731
Ident 731:736 text="float"
Gt 736:737
LParen 737:738
Number 738:741 text="2.5" suf="" isf=1 int=-
RParen 741:742
RParen 742:743
Dot 743:744
Ident 744:748 text="toBe"
LParen 748:749
Number 749:752 text="2.5" suf="" isf=1 int=-
RParen 752:753
Newline 753:754
RBrace 754:755
Newline 755:756
Newline 756:757
Ident 757:761 text="test"
Fn 762:764
Ident 765:780 text="generic_id_list"
LParen 780:781
RParen 781:782
LBrace 783:784
Newline 784:785
Ident 789:795 text="expect"
LParen 795:796
Ident 796:799 text="len"
LParen 799:800
Ident 800:805 text="id_fn"
Lt 805:806
Ident 806:810 text="List"
Lt 810:811
Ident 811:814 text="int"
Shr 814:816
LParen 816:817
LBracket 817:818
Number 818:819 text="1" suf="" isf=0 int=1
Comma 819:820
Number 821:822 text="2" suf="" isf=0 int=2
Comma 822:823
Number 824:825 text="3" suf="" isf=0 int=3
RBracket 825:826
RParen 826:827
RParen 827:828
RParen 828:829
Dot 829:830
Ident 830:834 text="toBe"
LParen 834:835
Number 835:836 text="3" suf="" isf=0 int=3
RParen 836:837
Newline 837:838
Ident 842:848 text="expect"
LParen 848:849
Ident 849:854 text="id_fn"
Lt 854:855
Ident 855:859 text="List"
Lt 859:860
Ident 860:863 text="int"
Shr 863:865
LParen 865:866
LBracket 866:867
Number 867:868 text="1" suf="" isf=0 int=1
Comma 868:869
Number 870:871 text="2" suf="" isf=0 int=2
Comma 871:872
Number 873:874 text="3" suf="" isf=0 int=3
RBracket 874:875
RParen 875:876
LBracket 876:877
Number 877:878 text="2" suf="" isf=0 int=2
RBracket 878:879
RParen 879:880
Dot 880:881
Ident 881:885 text="toBe"
LParen 885:886
Number 886:887 text="3" suf="" isf=0 int=3
RParen 887:888
Newline 888:889
RBrace 889:890
Newline 890:891
Newline 891:892
Ident 892:896 text="test"
Fn 897:899
Ident 900:921 text="generic_call_in_arith"
LParen 921:922
RParen 922:923
LBrace 924:925
Newline 925:926
Ident 930:936 text="expect"
LParen 936:937
Number 937:939 text="30" suf="" isf=0 int=30
Plus 940:941
Ident 942:947 text="id_fn"
Lt 947:948
Ident 948:951 text="int"
Gt 951:952
LParen 952:953
Number 953:955 text="12" suf="" isf=0 int=12
RParen 955:956
RParen 956:957
Dot 957:958
Ident 958:962 text="toBe"
LParen 962:963
Number 963:965 text="42" suf="" isf=0 int=42
RParen 965:966
Newline 966:967
RBrace 967:968
Newline 968:969
Newline 969:970
Ident 970:974 text="test"
Fn 975:977
Ident 978:998 text="nested_generic_calls"
LParen 998:999
RParen 999:1000
LBrace 1001:1002
Newline 1002:1003
Ident 1007:1013 text="expect"
LParen 1013:1014
Ident 1014:1019 text="twice"
Lt 1019:1020
Ident 1020:1023 text="int"
Gt 1023:1024
LParen 1024:1025
Number 1025:1027 text="21" suf="" isf=0 int=21
RParen 1027:1028
RParen 1028:1029
Dot 1029:1030
Ident 1030:1034 text="toBe"
LParen 1034:1035
Number 1035:1037 text="21" suf="" isf=0 int=21
RParen 1037:1038
Newline 1038:1039
Ident 1043:1049 text="expect"
LParen 1049:1050
Ident 1050:1055 text="twice"
Lt 1055:1056
Ident 1056:1062 text="string"
Gt 1062:1063
LParen 1063:1064
Str 1064:1068 T("ab")
RParen 1068:1069
RParen 1069:1070
Dot 1070:1071
Ident 1071:1075 text="toBe"
LParen 1075:1076
Str 1076:1080 T("ab")
RParen 1080:1081
Newline 1081:1082
Ident 1086:1092 text="expect"
LParen 1092:1093
Ident 1093:1098 text="twice"
Lt 1098:1099
Ident 1099:1104 text="float"
Gt 1104:1105
LParen 1105:1106
Number 1106:1109 text="3.5" suf="" isf=1 int=-
RParen 1109:1110
RParen 1110:1111
Dot 1111:1112
Ident 1112:1116 text="toBe"
LParen 1116:1117
Number 1117:1120 text="3.5" suf="" isf=1 int=-
RParen 1120:1121
Newline 1121:1122
RBrace 1122:1123
Newline 1123:1124
Newline 1124:1125
Ident 1125:1129 text="test"
Fn 1130:1132
Ident 1133:1154 text="generic_element_types"
LParen 1154:1155
RParen 1155:1156
LBrace 1157:1158
Newline 1158:1159
Ident 1163:1169 text="expect"
LParen 1169:1170
Ident 1170:1175 text="first"
Lt 1175:1176
Ident 1176:1179 text="int"
Gt 1179:1180
LParen 1180:1181
LBracket 1181:1182
Number 1182:1183 text="1" suf="" isf=0 int=1
Comma 1183:1184
Number 1185:1186 text="2" suf="" isf=0 int=2
Comma 1186:1187
Number 1188:1189 text="3" suf="" isf=0 int=3
RBracket 1189:1190
Comma 1190:1191
Number 1192:1193 text="0" suf="" isf=0 int=0
RParen 1193:1194
RParen 1194:1195
Dot 1195:1196
Ident 1196:1200 text="toBe"
LParen 1200:1201
Number 1201:1202 text="1" suf="" isf=0 int=1
RParen 1202:1203
Newline 1203:1204
Ident 1208:1214 text="expect"
LParen 1214:1215
Ident 1215:1220 text="first"
Lt 1220:1221
Ident 1221:1227 text="string"
Gt 1227:1228
LParen 1228:1229
LBracket 1229:1230
Str 1230:1233 T("a")
Comma 1233:1234
Str 1235:1238 T("b")
RBracket 1238:1239
Comma 1239:1240
Str 1241:1243
RParen 1243:1244
RParen 1244:1245
Dot 1245:1246
Ident 1246:1250 text="toBe"
LParen 1250:1251
Str 1251:1254 T("a")
RParen 1254:1255
Newline 1255:1256
Ident 1260:1266 text="expect"
LParen 1266:1267
Ident 1267:1272 text="first"
Lt 1272:1273
Ident 1273:1278 text="float"
Gt 1278:1279
LParen 1279:1280
LBracket 1280:1281
Number 1281:1284 text="3.5" suf="" isf=1 int=-
Comma 1284:1285
Number 1286:1289 text="2.5" suf="" isf=1 int=-
RBracket 1289:1290
Comma 1290:1291
Number 1292:1295 text="0.0" suf="" isf=1 int=-
RParen 1295:1296
RParen 1296:1297
Dot 1297:1298
Ident 1298:1302 text="toBe"
LParen 1302:1303
Number 1303:1306 text="3.5" suf="" isf=1 int=-
RParen 1306:1307
Newline 1307:1308
RBrace 1308:1309
Newline 1309:1310
Newline 1310:1311
Ident 1311:1315 text="test"
Fn 1316:1318
Ident 1319:1342 text="generic_two_type_params"
LParen 1342:1343
RParen 1343:1344
LBrace 1345:1346
Newline 1346:1347
Ident 1351:1357 text="expect"
LParen 1357:1358
Ident 1358:1364 text="choose"
Lt 1364:1365
Ident 1365:1368 text="int"
Comma 1368:1369
Ident 1370:1376 text="string"
Gt 1376:1377
LParen 1377:1378
Number 1378:1379 text="1" suf="" isf=0 int=1
Comma 1379:1380
Str 1381:1384 T("x")
RParen 1384:1385
RParen 1385:1386
Dot 1386:1387
Ident 1387:1391 text="toBe"
LParen 1391:1392
Str 1392:1395 T("x")
RParen 1395:1396
Newline 1396:1397
Ident 1401:1407 text="expect"
LParen 1407:1408
Ident 1408:1414 text="choose"
Lt 1414:1415
Ident 1415:1421 text="string"
Comma 1421:1422
Ident 1423:1428 text="float"
Gt 1428:1429
LParen 1429:1430
Str 1430:1433 T("a")
Comma 1433:1434
Number 1435:1438 text="1.5" suf="" isf=1 int=-
RParen 1438:1439
RParen 1439:1440
Dot 1440:1441
Ident 1441:1445 text="toBe"
LParen 1445:1446
Number 1446:1449 text="1.5" suf="" isf=1 int=-
RParen 1449:1450
Newline 1450:1451
RBrace 1451:1452
Newline 1452:1453
Newline 1453:1454
Ident 1454:1458 text="test"
Fn 1459:1461
Ident 1462:1490 text="generic_with_lambda_argument"
LParen 1490:1491
RParen 1491:1492
LBrace 1493:1494
Newline 1494:1495
Ident 1499:1505 text="expect"
LParen 1505:1506
Ident 1506:1517 text="apply_twice"
Lt 1517:1518
Ident 1518:1521 text="int"
Gt 1521:1522
LParen 1522:1523
Fn 1523:1525
LParen 1526:1527
Ident 1527:1528 text="n"
Colon 1528:1529
Ident 1530:1533 text="int"
RParen 1533:1534
Arrow 1535:1537
Ident 1538:1541 text="int"
LBrace 1542:1543
Ident 1544:1545 text="n"
Star 1546:1547
Number 1548:1549 text="2" suf="" isf=0 int=2
RBrace 1550:1551
Comma 1551:1552
Number 1553:1554 text="5" suf="" isf=0 int=5
RParen 1554:1555
RParen 1555:1556
Dot 1556:1557
Ident 1557:1561 text="toBe"
LParen 1561:1562
Number 1562:1564 text="20" suf="" isf=0 int=20
RParen 1564:1565
Newline 1565:1566
Ident 1570:1576 text="expect"
LParen 1576:1577
Ident 1577:1588 text="apply_twice"
Lt 1588:1589
Ident 1589:1592 text="int"
Gt 1592:1593
LParen 1593:1594
Ident 1594:1600 text="negate"
Comma 1600:1601
Number 1602:1603 text="3" suf="" isf=0 int=3
RParen 1603:1604
RParen 1604:1605
Dot 1605:1606
Ident 1606:1610 text="toBe"
LParen 1610:1611
Number 1611:1612 text="3" suf="" isf=0 int=3
RParen 1612:1613
Newline 1613:1614
RBrace 1614:1615
Newline 1615:1616
Newline 1616:1617
Ident 1617:1621 text="test"
Fn 1622:1624
Ident 1625:1641 text="inferred_id_ints"
LParen 1641:1642
RParen 1642:1643
LBrace 1644:1645
Newline 1645:1646
Ident 1650:1656 text="expect"
LParen 1656:1657
Ident 1657:1662 text="id_fn"
LParen 1662:1663
Number 1663:1664 text="3" suf="" isf=0 int=3
RParen 1664:1665
RParen 1665:1666
Dot 1666:1667
Ident 1667:1671 text="toBe"
LParen 1671:1672
Number 1672:1673 text="3" suf="" isf=0 int=3
RParen 1673:1674
Newline 1674:1675
Ident 1679:1685 text="expect"
LParen 1685:1686
Ident 1686:1691 text="id_fn"
LParen 1691:1692
Minus 1692:1693
Number 1693:1694 text="7" suf="" isf=0 int=7
RParen 1694:1695
RParen 1695:1696
Dot 1696:1697
Ident 1697:1701 text="toBe"
LParen 1701:1702
Minus 1702:1703
Number 1703:1704 text="7" suf="" isf=0 int=7
RParen 1704:1705
Newline 1705:1706
RBrace 1706:1707
Newline 1707:1708
Newline 1708:1709
Ident 1709:1713 text="test"
Fn 1714:1716
Ident 1717:1736 text="inferred_id_strings"
LParen 1736:1737
RParen 1737:1738
LBrace 1739:1740
Newline 1740:1741
Ident 1745:1751 text="expect"
LParen 1751:1752
Ident 1752:1757 text="id_fn"
LParen 1757:1758
Str 1758:1765 T("hello")
RParen 1765:1766
RParen 1766:1767
Dot 1767:1768
Ident 1768:1772 text="toBe"
LParen 1772:1773
Str 1773:1780 T("hello")
RParen 1780:1781
Newline 1781:1782
RBrace 1782:1783
Newline 1783:1784
Newline 1784:1785
Ident 1785:1789 text="test"
Fn 1790:1792
Ident 1793:1813 text="inferred_id_in_arith"
LParen 1813:1814
RParen 1814:1815
LBrace 1816:1817
Newline 1817:1818
Ident 1822:1828 text="expect"
LParen 1828:1829
Number 1829:1831 text="30" suf="" isf=0 int=30
Plus 1832:1833
Ident 1834:1839 text="id_fn"
LParen 1839:1840
Number 1840:1842 text="12" suf="" isf=0 int=12
RParen 1842:1843
RParen 1843:1844
Dot 1844:1845
Ident 1845:1849 text="toBe"
LParen 1849:1850
Number 1850:1852 text="42" suf="" isf=0 int=42
RParen 1852:1853
Newline 1853:1854
RBrace 1854:1855
Newline 1855:1856
Newline 1856:1857
Ident 1857:1861 text="test"
Fn 1862:1864
Ident 1865:1883 text="inferred_id_nested"
LParen 1883:1884
RParen 1884:1885
LBrace 1886:1887
Newline 1887:1888
Ident 1892:1898 text="expect"
LParen 1898:1899
Ident 1899:1904 text="id_fn"
LParen 1904:1905
Ident 1905:1910 text="id_fn"
LParen 1910:1911
Number 1911:1912 text="3" suf="" isf=0 int=3
RParen 1912:1913
RParen 1913:1914
RParen 1914:1915
Dot 1915:1916
Ident 1916:1920 text="toBe"
LParen 1920:1921
Number 1921:1922 text="3" suf="" isf=0 int=3
RParen 1922:1923
Newline 1923:1924
Ident 1928:1934 text="expect"
LParen 1934:1935
Ident 1935:1940 text="id_fn"
LParen 1940:1941
Ident 1941:1946 text="id_fn"
LParen 1946:1947
Str 1947:1950 T("x")
RParen 1950:1951
RParen 1951:1952
RParen 1952:1953
Dot 1953:1954
Ident 1954:1958 text="toBe"
LParen 1958:1959
Str 1959:1962 T("x")
RParen 1962:1963
Newline 1963:1964
RBrace 1964:1965
Newline 1965:1966
Newline 1966:1967
Ident 1967:1971 text="test"
Fn 1972:1974
Ident 1975:1991 text="inferred_id_list"
LParen 1991:1992
RParen 1992:1993
LBrace 1994:1995
Newline 1995:1996
Ident 2000:2006 text="expect"
LParen 2006:2007
Ident 2007:2010 text="len"
LParen 2010:2011
Ident 2011:2016 text="id_fn"
LParen 2016:2017
LBracket 2017:2018
Number 2018:2019 text="1" suf="" isf=0 int=1
Comma 2019:2020
Number 2021:2022 text="2" suf="" isf=0 int=2
Comma 2022:2023
Number 2024:2025 text="3" suf="" isf=0 int=3
RBracket 2025:2026
RParen 2026:2027
RParen 2027:2028
RParen 2028:2029
Dot 2029:2030
Ident 2030:2034 text="toBe"
LParen 2034:2035
Number 2035:2036 text="3" suf="" isf=0 int=3
RParen 2036:2037
Newline 2037:2038
Ident 2042:2048 text="expect"
LParen 2048:2049
Ident 2049:2054 text="id_fn"
LParen 2054:2055
LBracket 2055:2056
Number 2056:2057 text="1" suf="" isf=0 int=1
Comma 2057:2058
Number 2059:2060 text="2" suf="" isf=0 int=2
Comma 2060:2061
Number 2062:2063 text="3" suf="" isf=0 int=3
RBracket 2063:2064
RParen 2064:2065
LBracket 2065:2066
Number 2066:2067 text="2" suf="" isf=0 int=2
RBracket 2067:2068
RParen 2068:2069
Dot 2069:2070
Ident 2070:2074 text="toBe"
LParen 2074:2075
Number 2075:2076 text="3" suf="" isf=0 int=3
RParen 2076:2077
Newline 2077:2078
RBrace 2078:2079
Newline 2079:2080
Newline 2080:2081
Ident 2081:2085 text="test"
Fn 2086:2088
Ident 2089:2103 text="inferred_twice"
LParen 2103:2104
RParen 2104:2105
LBrace 2106:2107
Newline 2107:2108
Ident 2112:2118 text="expect"
LParen 2118:2119
Ident 2119:2124 text="twice"
LParen 2124:2125
Number 2125:2127 text="21" suf="" isf=0 int=21
RParen 2127:2128
RParen 2128:2129
Dot 2129:2130
Ident 2130:2134 text="toBe"
LParen 2134:2135
Number 2135:2137 text="21" suf="" isf=0 int=21
RParen 2137:2138
Newline 2138:2139
Ident 2143:2149 text="expect"
LParen 2149:2150
Ident 2150:2155 text="twice"
LParen 2155:2156
Str 2156:2160 T("ab")
RParen 2160:2161
RParen 2161:2162
Dot 2162:2163
Ident 2163:2167 text="toBe"
LParen 2167:2168
Str 2168:2172 T("ab")
RParen 2172:2173
Newline 2173:2174
RBrace 2174:2175
Newline 2175:2176
Newline 2176:2177
Ident 2177:2181 text="test"
Fn 2182:2184
Ident 2185:2199 text="inferred_first"
LParen 2199:2200
RParen 2200:2201
LBrace 2202:2203
Newline 2203:2204
Ident 2208:2214 text="expect"
LParen 2214:2215
Ident 2215:2220 text="first"
LParen 2220:2221
LBracket 2221:2222
Number 2222:2223 text="1" suf="" isf=0 int=1
Comma 2223:2224
Number 2225:2226 text="2" suf="" isf=0 int=2
Comma 2226:2227
Number 2228:2229 text="3" suf="" isf=0 int=3
RBracket 2229:2230
Comma 2230:2231
Number 2232:2233 text="0" suf="" isf=0 int=0
RParen 2233:2234
RParen 2234:2235
Dot 2235:2236
Ident 2236:2240 text="toBe"
LParen 2240:2241
Number 2241:2242 text="1" suf="" isf=0 int=1
RParen 2242:2243
Newline 2243:2244
Ident 2248:2254 text="expect"
LParen 2254:2255
Ident 2255:2260 text="first"
LParen 2260:2261
LBracket 2261:2262
Str 2262:2265 T("a")
Comma 2265:2266
Str 2267:2270 T("b")
RBracket 2270:2271
Comma 2271:2272
Str 2273:2275
RParen 2275:2276
RParen 2276:2277
Dot 2277:2278
Ident 2278:2282 text="toBe"
LParen 2282:2283
Str 2283:2286 T("a")
RParen 2286:2287
Newline 2287:2288
Ident 2292:2298 text="expect"
LParen 2298:2299
Ident 2299:2304 text="first"
LParen 2304:2305
LBracket 2305:2306
Number 2306:2309 text="3.5" suf="" isf=1 int=-
Comma 2309:2310
Number 2311:2314 text="2.5" suf="" isf=1 int=-
RBracket 2314:2315
Comma 2315:2316
Number 2317:2320 text="0.0" suf="" isf=1 int=-
RParen 2320:2321
RParen 2321:2322
Dot 2322:2323
Ident 2323:2327 text="toBe"
LParen 2327:2328
Number 2328:2331 text="3.5" suf="" isf=1 int=-
RParen 2331:2332
Newline 2332:2333
RBrace 2333:2334
Newline 2334:2335
Newline 2335:2336
Ident 2336:2340 text="test"
Fn 2341:2343
Ident 2344:2359 text="inferred_choose"
LParen 2359:2360
RParen 2360:2361
LBrace 2362:2363
Newline 2363:2364
Ident 2368:2374 text="expect"
LParen 2374:2375
Ident 2375:2381 text="choose"
LParen 2381:2382
Number 2382:2383 text="1" suf="" isf=0 int=1
Comma 2383:2384
Str 2385:2388 T("x")
RParen 2388:2389
RParen 2389:2390
Dot 2390:2391
Ident 2391:2395 text="toBe"
LParen 2395:2396
Str 2396:2399 T("x")
RParen 2399:2400
Newline 2400:2401
Ident 2405:2411 text="expect"
LParen 2411:2412
Ident 2412:2418 text="choose"
LParen 2418:2419
Str 2419:2422 T("a")
Comma 2422:2423
Number 2424:2427 text="1.5" suf="" isf=1 int=-
RParen 2427:2428
RParen 2428:2429
Dot 2429:2430
Ident 2430:2434 text="toBe"
LParen 2434:2435
Number 2435:2438 text="1.5" suf="" isf=1 int=-
RParen 2438:2439
Newline 2439:2440
RBrace 2440:2441
Newline 2441:2442
Newline 2442:2443
Ident 2443:2447 text="test"
Fn 2448:2450
Ident 2451:2471 text="inferred_apply_twice"
LParen 2471:2472
RParen 2472:2473
LBrace 2474:2475
Newline 2475:2476
Ident 2480:2486 text="expect"
LParen 2486:2487
Ident 2487:2498 text="apply_twice"
LParen 2498:2499
Fn 2499:2501
LParen 2502:2503
Ident 2503:2504 text="n"
Colon 2504:2505
Ident 2506:2509 text="int"
RParen 2509:2510
Arrow 2511:2513
Ident 2514:2517 text="int"
LBrace 2518:2519
Ident 2520:2521 text="n"
Star 2522:2523
Number 2524:2525 text="2" suf="" isf=0 int=2
RBrace 2526:2527
Comma 2527:2528
Number 2529:2530 text="5" suf="" isf=0 int=5
RParen 2530:2531
RParen 2531:2532
Dot 2532:2533
Ident 2533:2537 text="toBe"
LParen 2537:2538
Number 2538:2540 text="20" suf="" isf=0 int=20
RParen 2540:2541
Newline 2541:2542
Ident 2546:2552 text="expect"
LParen 2552:2553
Ident 2553:2564 text="apply_twice"
LParen 2564:2565
Ident 2565:2571 text="negate"
Comma 2571:2572
Number 2573:2574 text="3" suf="" isf=0 int=3
RParen 2574:2575
RParen 2575:2576
Dot 2576:2577
Ident 2577:2581 text="toBe"
LParen 2581:2582
Number 2582:2583 text="3" suf="" isf=0 int=3
RParen 2583:2584
Newline 2584:2585
RBrace 2585:2586
Newline 2586:2587
Newline 2587:2588
Ident 2588:2592 text="test"
Fn 2593:2595
Ident 2596:2617 text="inferred_option_param"
LParen 2617:2618
RParen 2618:2619
LBrace 2620:2621
Newline 2621:2622
Ident 2626:2632 text="expect"
LParen 2632:2633
Ident 2633:2641 text="pick_one"
LParen 2641:2642
LBracket 2642:2643
Number 2643:2644 text="1" suf="" isf=0 int=1
Comma 2644:2645
Number 2646:2647 text="2" suf="" isf=0 int=2
RBracket 2647:2648
Comma 2648:2649
None 2650:2654
RParen 2654:2655
RParen 2655:2656
Dot 2656:2657
Ident 2657:2661 text="toBe"
LParen 2661:2662
Number 2662:2663 text="1" suf="" isf=0 int=1
RParen 2663:2664
Newline 2664:2665
Ident 2669:2675 text="expect"
LParen 2675:2676
Ident 2676:2684 text="pick_one"
LParen 2684:2685
LBracket 2685:2686
Number 2686:2687 text="9" suf="" isf=0 int=9
RBracket 2687:2688
Comma 2688:2689
Number 2690:2691 text="5" suf="" isf=0 int=5
RParen 2691:2692
RParen 2692:2693
Dot 2693:2694
Ident 2694:2698 text="toBe"
LParen 2698:2699
Number 2699:2700 text="9" suf="" isf=0 int=9
RParen 2700:2701
Newline 2701:2702
Ident 2706:2712 text="expect"
LParen 2712:2713
Ident 2713:2721 text="pick_one"
LParen 2721:2722
LBracket 2722:2723
Str 2723:2726 T("a")
Comma 2726:2727
Str 2728:2731 T("b")
RBracket 2731:2732
Comma 2732:2733
None 2734:2738
RParen 2738:2739
RParen 2739:2740
Dot 2740:2741
Ident 2741:2745 text="toBe"
LParen 2745:2746
Str 2746:2749 T("a")
RParen 2749:2750
Newline 2750:2751
RBrace 2751:2752
Eof 2752:2752
