Class 0:5
Ident 6:12 text="Animal"
LBrace 13:14
Newline 14:15
Fn 19:21
Ident 22:30 text="describe"
LParen 30:31
RParen 31:32
Arrow 33:35
Ident 36:42 text="string"
LBrace 43:44
Newline 44:45
Return 53:59
Str 60:68 T("animal")
Newline 68:69
RBrace 73:74
Newline 74:75
Newline 75:76
Fn 80:82
Ident 83:86 text="age"
LParen 86:87
RParen 87:88
Arrow 89:91
Ident 92:95 text="int"
LBrace 96:97
Newline 97:98
Return 106:112
Number 113:114 text="3" suf="" isf=0 int=3
Newline 114:115
RBrace 119:120
Newline 120:121
RBrace 121:122
Newline 122:123
Newline 123:124
Class 124:129
Ident 130:133 text="Dog"
Extends 134:141
Ident 142:148 text="Animal"
LBrace 149:150
Newline 150:151
Override 155:163
Fn 164:166
Ident 167:175 text="describe"
LParen 175:176
RParen 176:177
Arrow 178:180
Ident 181:187 text="string"
LBrace 188:189
Newline 189:190
Return 198:204
Str 205:210 T("dog")
Newline 210:211
RBrace 215:216
Newline 216:217
Newline 217:218
Fn 222:224
Ident 225:238 text="superDescribe"
LParen 238:239
RParen 239:240
Arrow 241:243
Ident 244:250 text="string"
LBrace 251:252
Newline 252:253
Return 261:267
Super 268:273
Dot 273:274
Ident 274:282 text="describe"
LParen 282:283
RParen 283:284
Newline 284:285
RBrace 289:290
Newline 290:291
RBrace 291:292
Newline 292:293
Newline 293:294
Class 294:299
Ident 300:310 text="WorkingDog"
Extends 311:318
Ident 319:322 text="Dog"
LBrace 323:324
Newline 324:325
RBrace 325:326
Newline 326:327
Newline 327:328
Class 328:333
Ident 334:340 text="Poodle"
Extends 341:348
Ident 349:359 text="WorkingDog"
LBrace 360:361
Newline 361:362
Override 366:374
Fn 375:377
Ident 378:386 text="describe"
LParen 386:387
RParen 387:388
Arrow 389:391
Ident 392:398 text="string"
LBrace 399:400
Newline 400:401
Return 409:415
Str 416:424 T("poodle")
Newline 424:425
RBrace 429:430
Newline 430:431
RBrace 431:432
Newline 432:433
Newline 433:434
Ident 434:442 text="describe"
LParen 442:443
Str 443:469 T("method override dispatch")
Comma 469:470
LBrace 471:472
Newline 472:473
Ident 477:481 text="test"
LParen 481:482
Str 482:531 T("receiver statically typed as the defining class")
Comma 531:532
LBrace 533:534
Newline 534:535
Let 543:546
Ident 547:548 text="a"
Colon 548:549
Ident 550:556 text="Animal"
Assign 557:558
Ident 559:565 text="Animal"
LParen 565:566
RParen 566:567
Newline 567:568
Ident 576:582 text="expect"
LParen 582:583
Ident 583:584 text="a"
Dot 584:585
Ident 585:593 text="describe"
LParen 593:594
RParen 594:595
RParen 595:596
Dot 596:597
Ident 597:601 text="toBe"
LParen 601:602
Str 602:610 T("animal")
RParen 610:611
Newline 611:612
RBrace 616:617
RParen 617:618
Newline 618:619
Newline 619:620
Ident 624:628 text="test"
LParen 628:629
Str 629:671 T("receiver statically typed as an ancestor")
Comma 671:672
LBrace 673:674
Newline 674:675
Let 683:686
Ident 687:688 text="d"
Colon 688:689
Ident 690:696 text="Animal"
Assign 697:698
Ident 699:702 text="Dog"
LParen 702:703
RParen 703:704
Newline 704:705
Ident 713:719 text="expect"
LParen 719:720
Ident 720:721 text="d"
Dot 721:722
Ident 722:730 text="describe"
LParen 730:731
RParen 731:732
RParen 732:733
Dot 733:734
Ident 734:738 text="toBe"
LParen 738:739
Str 739:744 T("dog")
RParen 744:745
Newline 745:746
Let 754:757
Ident 758:759 text="p"
Colon 759:760
Ident 761:767 text="Animal"
Assign 768:769
Ident 770:776 text="Poodle"
LParen 776:777
RParen 777:778
Newline 778:779
Ident 787:793 text="expect"
LParen 793:794
Ident 794:795 text="p"
Dot 795:796
Ident 796:804 text="describe"
LParen 804:805
RParen 805:806
RParen 806:807
Dot 807:808
Ident 808:812 text="toBe"
LParen 812:813
Str 813:821 T("poodle")
RParen 821:822
Newline 822:823
RBrace 827:828
RParen 828:829
Newline 829:830
Newline 830:831
Ident 835:839 text="test"
LParen 839:840
Str 840:894 T("receiver statically typed as an inheriting mid-class")
Comma 894:895
LBrace 896:897
Newline 897:898
Newline 972:973
Newline 1047:1048
Newline 1123:1124
Let 1132:1135
Ident 1136:1137 text="w"
Colon 1137:1138
Ident 1139:1149 text="WorkingDog"
Assign 1150:1151
Ident 1152:1162 text="WorkingDog"
LParen 1162:1163
RParen 1163:1164
Newline 1164:1165
Ident 1173:1179 text="expect"
LParen 1179:1180
Ident 1180:1181 text="w"
Dot 1181:1182
Ident 1182:1190 text="describe"
LParen 1190:1191
RParen 1191:1192
RParen 1192:1193
Dot 1193:1194
Ident 1194:1198 text="toBe"
LParen 1198:1199
Str 1199:1204 T("dog")
RParen 1204:1205
Newline 1205:1206
Let 1214:1217
Ident 1218:1220 text="wp"
Colon 1220:1221
Ident 1222:1232 text="WorkingDog"
Assign 1233:1234
Ident 1235:1241 text="Poodle"
LParen 1241:1242
RParen 1242:1243
Newline 1243:1244
Ident 1252:1258 text="expect"
LParen 1258:1259
Ident 1259:1261 text="wp"
Dot 1261:1262
Ident 1262:1270 text="describe"
LParen 1270:1271
RParen 1271:1272
RParen 1272:1273
Dot 1273:1274
Ident 1274:1278 text="toBe"
LParen 1278:1279
Str 1279:1287 T("poodle")
RParen 1287:1288
Newline 1288:1289
RBrace 1293:1294
RParen 1294:1295
Newline 1295:1296
Newline 1296:1297
Ident 1301:1305 text="test"
LParen 1305:1306
Str 1306:1341 T("override down a three-level chain")
Comma 1341:1342
LBrace 1343:1344
Newline 1344:1345
Let 1353:1356
Ident 1357:1358 text="p"
Colon 1358:1359
Ident 1360:1366 text="Poodle"
Assign 1367:1368
Ident 1369:1375 text="Poodle"
LParen 1375:1376
RParen 1376:1377
Newline 1377:1378
Ident 1386:1392 text="expect"
LParen 1392:1393
Ident 1393:1394 text="p"
Dot 1394:1395
Ident 1395:1403 text="describe"
LParen 1403:1404
RParen 1404:1405
RParen 1405:1406
Dot 1406:1407
Ident 1407:1411 text="toBe"
LParen 1411:1412
Str 1412:1420 T("poodle")
RParen 1420:1421
Newline 1421:1422
Let 1430:1433
Ident 1434:1435 text="a"
Colon 1435:1436
Ident 1437:1443 text="Animal"
Assign 1444:1445
Ident 1446:1447 text="p"
Newline 1447:1448
Ident 1456:1462 text="expect"
LParen 1462:1463
Ident 1463:1464 text="a"
Dot 1464:1465
Ident 1465:1473 text="describe"
LParen 1473:1474
RParen 1474:1475
RParen 1475:1476
Dot 1476:1477
Ident 1477:1481 text="toBe"
LParen 1481:1482
Str 1482:1490 T("poodle")
RParen 1490:1491
Newline 1491:1492
RBrace 1496:1497
RParen 1497:1498
Newline 1498:1499
Newline 1499:1500
Ident 1504:1508 text="test"
LParen 1508:1509
Str 1509:1557 T("super call binds the superclass implementation")
Comma 1557:1558
LBrace 1559:1560
Newline 1560:1561
Ident 1569:1575 text="expect"
LParen 1575:1576
Ident 1576:1579 text="Dog"
LParen 1579:1580
RParen 1580:1581
Dot 1581:1582
Ident 1582:1595 text="superDescribe"
LParen 1595:1596
RParen 1596:1597
RParen 1597:1598
Dot 1598:1599
Ident 1599:1603 text="toBe"
LParen 1603:1604
Str 1604:1612 T("animal")
RParen 1612:1613
Newline 1613:1614
Ident 1622:1628 text="expect"
LParen 1628:1629
Ident 1629:1635 text="Poodle"
LParen 1635:1636
RParen 1636:1637
Dot 1637:1638
Ident 1638:1651 text="superDescribe"
LParen 1651:1652
RParen 1652:1653
RParen 1653:1654
Dot 1654:1655
Ident 1655:1659 text="toBe"
LParen 1659:1660
Str 1660:1668 T("animal")
RParen 1668:1669
Newline 1669:1670
RBrace 1674:1675
RParen 1675:1676
Newline 1676:1677
Newline 1677:1678
Ident 1682:1686 text="test"
LParen 1686:1687
Str 1687:1727 T("never-overridden methods call directly")
Comma 1727:1728
LBrace 1729:1730
Newline 1730:1731
Let 1739:1742
Ident 1743:1744 text="a"
Colon 1744:1745
Ident 1746:1752 text="Animal"
Assign 1753:1754
Ident 1755:1761 text="Poodle"
LParen 1761:1762
RParen 1762:1763
Newline 1763:1764
Ident 1772:1778 text="expect"
LParen 1778:1779
Ident 1779:1780 text="a"
Dot 1780:1781
Ident 1781:1784 text="age"
LParen 1784:1785
RParen 1785:1786
RParen 1786:1787
Dot 1787:1788
Ident 1788:1792 text="toBe"
LParen 1792:1793
Number 1793:1794 text="3" suf="" isf=0 int=3
RParen 1794:1795
Newline 1795:1796
RBrace 1800:1801
RParen 1801:1802
Newline 1802:1803
RBrace 1803:1804
RParen 1804:1805
Eof 1805:1805
