Ident 0:4 text="test"
Fn 5:7
Ident 8:16 text="abs_ints"
LParen 16:17
RParen 17:18
LBrace 19:20
Newline 20:21
Ident 25:31 text="expect"
LParen 31:32
Ident 32:35 text="abs"
LParen 35:36
Number 36:37 text="5" suf="" isf=0 int=5
RParen 37:38
RParen 38:39
Dot 39:40
Ident 40:44 text="toBe"
LParen 44:45
Number 45:46 text="5" suf="" isf=0 int=5
RParen 46:47
Newline 47:48
Ident 52:58 text="expect"
LParen 58:59
Ident 59:62 text="abs"
LParen 62:63
Number 63:64 text="0" suf="" isf=0 int=0
RParen 64:65
RParen 65:66
Dot 66:67
Ident 67:71 text="toBe"
LParen 71:72
Number 72:73 text="0" suf="" isf=0 int=0
RParen 73:74
Newline 74:75
Ident 79:85 text="expect"
LParen 85:86
Ident 86:89 text="abs"
LParen 89:90
Minus 90:91
Number 91:92 text="5" suf="" isf=0 int=5
RParen 92:93
RParen 93:94
Dot 94:95
Ident 95:99 text="toBe"
LParen 99:100
Number 100:101 text="5" suf="" isf=0 int=5
RParen 101:102
Newline 102:103
RBrace 103:104
Newline 104:105
Newline 105:106
Ident 106:110 text="test"
Fn 111:113
Ident 114:124 text="abs_floats"
LParen 124:125
RParen 125:126
LBrace 127:128
Newline 128:129
Ident 133:139 text="expect"
LParen 139:140
Ident 140:143 text="abs"
LParen 143:144
Number 144:147 text="2.5" suf="" isf=1 int=-
RParen 147:148
RParen 148:149
Dot 149:150
Ident 150:154 text="toBe"
LParen 154:155
Number 155:158 text="2.5" suf="" isf=1 int=-
RParen 158:159
Newline 159:160
Ident 164:170 text="expect"
LParen 170:171
Ident 171:174 text="abs"
LParen 174:175
Minus 175:176
Number 176:179 text="2.5" suf="" isf=1 int=-
RParen 179:180
RParen 180:181
Dot 181:182
Ident 182:186 text="toBe"
LParen 186:187
Number 187:190 text="2.5" suf="" isf=1 int=-
RParen 190:191
Newline 191:192
RBrace 192:193
Newline 193:194
Newline 194:195
Ident 195:199 text="test"
Fn 200:202
Ident 203:215 text="range_simple"
LParen 215:216
RParen 216:217
LBrace 218:219
Newline 219:220
Let 224:227
Ident 228:229 text="r"
Assign 230:231
Ident 232:237 text="range"
LParen 237:238
Number 238:239 text="5" suf="" isf=0 int=5
RParen 239:240
Newline 240:241
Ident 245:251 text="expect"
LParen 251:252
Ident 252:255 text="len"
LParen 255:256
Ident 256:257 text="r"
RParen 257:258
RParen 258:259
Dot 259:260
Ident 260:264 text="toBe"
LParen 264:265
Number 265:266 text="5" suf="" isf=0 int=5
RParen 266:267
Newline 267:268
Ident 272:278 text="expect"
LParen 278:279
Ident 279:280 text="r"
LBracket 280:281
Number 281:282 text="0" suf="" isf=0 int=0
RBracket 282:283
RParen 283:284
Dot 284:285
Ident 285:289 text="toBe"
LParen 289:290
Number 290:291 text="0" suf="" isf=0 int=0
RParen 291:292
Newline 292:293
Ident 297:303 text="expect"
LParen 303:304
Ident 304:305 text="r"
LBracket 305:306
Number 306:307 text="4" suf="" isf=0 int=4
RBracket 307:308
RParen 308:309
Dot 309:310
Ident 310:314 text="toBe"
LParen 314:315
Number 315:316 text="4" suf="" isf=0 int=4
RParen 316:317
Newline 317:318
RBrace 318:319
Newline 319:320
Newline 320:321
Ident 321:325 text="test"
Fn 326:328
Ident 329:341 text="range_bounds"
LParen 341:342
RParen 342:343
LBrace 344:345
Newline 345:346
Let 350:353
Ident 354:355 text="r"
Assign 356:357
Ident 358:363 text="range"
LParen 363:364
Number 364:365 text="2" suf="" isf=0 int=2
Comma 365:366
Number 367:368 text="5" suf="" isf=0 int=5
RParen 368:369
Newline 369:370
Ident 374:380 text="expect"
LParen 380:381
Ident 381:384 text="len"
LParen 384:385
Ident 385:386 text="r"
RParen 386:387
RParen 387:388
Dot 388:389
Ident 389:393 text="toBe"
LParen 393:394
Number 394:395 text="3" suf="" isf=0 int=3
RParen 395:396
Newline 396:397
Ident 401:407 text="expect"
LParen 407:408
Ident 408:409 text="r"
LBracket 409:410
Number 410:411 text="0" suf="" isf=0 int=0
RBracket 411:412
RParen 412:413
Dot 413:414
Ident 414:418 text="toBe"
LParen 418:419
Number 419:420 text="2" suf="" isf=0 int=2
RParen 420:421
Newline 421:422
Ident 426:432 text="expect"
LParen 432:433
Ident 433:434 text="r"
LBracket 434:435
Number 435:436 text="2" suf="" isf=0 int=2
RBracket 436:437
RParen 437:438
Dot 438:439
Ident 439:443 text="toBe"
LParen 443:444
Number 444:445 text="4" suf="" isf=0 int=4
RParen 445:446
Newline 446:447
RBrace 447:448
Newline 448:449
Newline 449:450
Ident 450:454 text="test"
Fn 455:457
Ident 458:468 text="range_step"
LParen 468:469
RParen 469:470
LBrace 471:472
Newline 472:473
Let 477:480
Ident 481:482 text="r"
Assign 483:484
Ident 485:490 text="range"
LParen 490:491
Number 491:492 text="1" suf="" isf=0 int=1
Comma 492:493
Number 494:495 text="7" suf="" isf=0 int=7
Comma 495:496
Number 497:498 text="2" suf="" isf=0 int=2
RParen 498:499
Newline 499:500
Ident 504:510 text="expect"
LParen 510:511
Ident 511:514 text="len"
LParen 514:515
Ident 515:516 text="r"
RParen 516:517
RParen 517:518
Dot 518:519
Ident 519:523 text="toBe"
LParen 523:524
Number 524:525 text="3" suf="" isf=0 int=3
RParen 525:526
Newline 526:527
Ident 531:537 text="expect"
LParen 537:538
Ident 538:539 text="r"
LBracket 539:540
Number 540:541 text="0" suf="" isf=0 int=0
RBracket 541:542
RParen 542:543
Dot 543:544
Ident 544:548 text="toBe"
LParen 548:549
Number 549:550 text="1" suf="" isf=0 int=1
RParen 550:551
Newline 551:552
Ident 556:562 text="expect"
LParen 562:563
Ident 563:564 text="r"
LBracket 564:565
Number 565:566 text="1" suf="" isf=0 int=1
RBracket 566:567
RParen 567:568
Dot 568:569
Ident 569:573 text="toBe"
LParen 573:574
Number 574:575 text="3" suf="" isf=0 int=3
RParen 575:576
Newline 576:577
Ident 581:587 text="expect"
LParen 587:588
Ident 588:589 text="r"
LBracket 589:590
Number 590:591 text="2" suf="" isf=0 int=2
RBracket 591:592
RParen 592:593
Dot 593:594
Ident 594:598 text="toBe"
LParen 598:599
Number 599:600 text="5" suf="" isf=0 int=5
RParen 600:601
Newline 601:602
RBrace 602:603
Newline 603:604
Newline 604:605
Ident 605:609 text="test"
Fn 610:612
Ident 613:632 text="range_step_negative"
LParen 632:633
RParen 633:634
LBrace 635:636
Newline 636:637
Let 641:644
Ident 645:646 text="r"
Assign 647:648
Ident 649:654 text="range"
LParen 654:655
Number 655:656 text="5" suf="" isf=0 int=5
Comma 656:657
Number 658:659 text="0" suf="" isf=0 int=0
Comma 659:660
Minus 661:662
Number 662:663 text="2" suf="" isf=0 int=2
RParen 663:664
Newline 664:665
Ident 669:675 text="expect"
LParen 675:676
Ident 676:679 text="len"
LParen 679:680
Ident 680:681 text="r"
RParen 681:682
RParen 682:683
Dot 683:684
Ident 684:688 text="toBe"
LParen 688:689
Number 689:690 text="3" suf="" isf=0 int=3
RParen 690:691
Newline 691:692
Ident 696:702 text="expect"
LParen 702:703
Ident 703:704 text="r"
LBracket 704:705
Number 705:706 text="0" suf="" isf=0 int=0
RBracket 706:707
RParen 707:708
Dot 708:709
Ident 709:713 text="toBe"
LParen 713:714
Number 714:715 text="5" suf="" isf=0 int=5
RParen 715:716
Newline 716:717
Ident 721:727 text="expect"
LParen 727:728
Ident 728:729 text="r"
LBracket 729:730
Number 730:731 text="1" suf="" isf=0 int=1
RBracket 731:732
RParen 732:733
Dot 733:734
Ident 734:738 text="toBe"
LParen 738:739
Number 739:740 text="3" suf="" isf=0 int=3
RParen 740:741
Newline 741:742
Ident 746:752 text="expect"
LParen 752:753
Ident 753:754 text="r"
LBracket 754:755
Number 755:756 text="2" suf="" isf=0 int=2
RBracket 756:757
RParen 757:758
Dot 758:759
Ident 759:763 text="toBe"
LParen 763:764
Number 764:765 text="1" suf="" isf=0 int=1
RParen 765:766
Newline 766:767
RBrace 767:768
Newline 768:769
Newline 769:770
Ident 770:774 text="test"
Fn 775:777
Ident 778:793 text="range_zero_step"
LParen 793:794
RParen 794:795
LBrace 796:797
Newline 797:798
Let 802:805
Ident 806:807 text="r"
Assign 808:809
Ident 810:815 text="range"
LParen 815:816
Number 816:817 text="0" suf="" isf=0 int=0
Comma 817:818
Number 819:821 text="10" suf="" isf=0 int=10
Comma 821:822
Number 823:824 text="0" suf="" isf=0 int=0
RParen 824:825
Newline 825:826
Ident 830:836 text="expect"
LParen 836:837
Ident 837:840 text="len"
LParen 840:841
Ident 841:842 text="r"
RParen 842:843
RParen 843:844
Dot 844:845
Ident 845:849 text="toBe"
LParen 849:850
Number 850:851 text="0" suf="" isf=0 int=0
RParen 851:852
Newline 852:853
RBrace 853:854
Newline 854:855
Newline 855:856
Ident 856:860 text="test"
Fn 861:863
Ident 864:875 text="range_empty"
LParen 875:876
RParen 876:877
LBrace 878:879
Newline 879:880
Let 884:887
Ident 888:889 text="r"
Assign 890:891
Ident 892:897 text="range"
LParen 897:898
Number 898:900 text="10" suf="" isf=0 int=10
Comma 900:901
Number 902:903 text="0" suf="" isf=0 int=0
RParen 903:904
Newline 904:905
Ident 909:915 text="expect"
LParen 915:916
Ident 916:919 text="len"
LParen 919:920
Ident 920:921 text="r"
RParen 921:922
RParen 922:923
Dot 923:924
Ident 924:928 text="toBe"
LParen 928:929
Number 929:930 text="0" suf="" isf=0 int=0
RParen 930:931
Newline 931:932
RBrace 932:933
Newline 933:934
Newline 934:935
Ident 935:939 text="test"
Fn 940:942
Ident 943:955 text="for_in_range"
LParen 955:956
RParen 956:957
LBrace 958:959
Newline 959:960
Var 964:967
Ident 968:971 text="sum"
Assign 972:973
Number 974:975 text="0" suf="" isf=0 int=0
Newline 975:976
For 980:983
LParen 984:985
Ident 985:986 text="x"
In 987:989
Ident 990:995 text="range"
LParen 995:996
Number 996:997 text="5" suf="" isf=0 int=5
RParen 997:998
RParen 998:999
LBrace 1000:1001
Newline 1001:1002
Ident 1010:1013 text="sum"
PlusEq 1014:1016
Ident 1017:1018 text="x"
Newline 1018:1019
RBrace 1023:1024
Newline 1024:1025
Ident 1029:1035 text="expect"
LParen 1035:1036
Ident 1036:1039 text="sum"
RParen 1039:1040
Dot 1040:1041
Ident 1041:1045 text="toBe"
LParen 1045:1046
Number 1046:1048 text="10" suf="" isf=0 int=10
RParen 1048:1049
Newline 1049:1050
RBrace 1050:1051
Newline 1051:1052
Newline 1052:1053
Ident 1053:1057 text="test"
Fn 1058:1060
Ident 1061:1078 text="for_in_range_step"
LParen 1078:1079
RParen 1079:1080
LBrace 1081:1082
Newline 1082:1083
Var 1087:1090
Ident 1091:1094 text="sum"
Assign 1095:1096
Number 1097:1098 text="0" suf="" isf=0 int=0
Newline 1098:1099
For 1103:1106
LParen 1107:1108
Ident 1108:1109 text="x"
In 1110:1112
Ident 1113:1118 text="range"
LParen 1118:1119
Number 1119:1120 text="1" suf="" isf=0 int=1
Comma 1120:1121
Number 1122:1124 text="10" suf="" isf=0 int=10
Comma 1124:1125
Number 1126:1127 text="2" suf="" isf=0 int=2
RParen 1127:1128
RParen 1128:1129
LBrace 1130:1131
Newline 1131:1132
Ident 1140:1143 text="sum"
PlusEq 1144:1146
Ident 1147:1148 text="x"
Newline 1148:1149
RBrace 1153:1154
Newline 1154:1155
Ident 1159:1165 text="expect"
LParen 1165:1166
Ident 1166:1169 text="sum"
RParen 1169:1170
Dot 1170:1171
Ident 1171:1175 text="toBe"
LParen 1175:1176
Number 1176:1178 text="25" suf="" isf=0 int=25
RParen 1178:1179
Newline 1179:1180
RBrace 1180:1181
Newline 1181:1182
Newline 1182:1183
Ident 1183:1187 text="test"
Fn 1188:1190
Ident 1191:1199 text="min_ints"
LParen 1199:1200
RParen 1200:1201
LBrace 1202:1203
Newline 1203:1204
Ident 1208:1214 text="expect"
LParen 1214:1215
Ident 1215:1218 text="min"
LParen 1218:1219
Number 1219:1220 text="3" suf="" isf=0 int=3
Comma 1220:1221
Number 1222:1223 text="7" suf="" isf=0 int=7
RParen 1223:1224
RParen 1224:1225
Dot 1225:1226
Ident 1226:1230 text="toBe"
LParen 1230:1231
Number 1231:1232 text="3" suf="" isf=0 int=3
RParen 1232:1233
Newline 1233:1234
Ident 1238:1244 text="expect"
LParen 1244:1245
Ident 1245:1248 text="min"
LParen 1248:1249
Number 1249:1250 text="7" suf="" isf=0 int=7
Comma 1250:1251
Number 1252:1253 text="3" suf="" isf=0 int=3
RParen 1253:1254
RParen 1254:1255
Dot 1255:1256
Ident 1256:1260 text="toBe"
LParen 1260:1261
Number 1261:1262 text="3" suf="" isf=0 int=3
RParen 1262:1263
Newline 1263:1264
Ident 1268:1274 text="expect"
LParen 1274:1275
Ident 1275:1278 text="min"
LParen 1278:1279
Minus 1279:1280
Number 1280:1281 text="2" suf="" isf=0 int=2
Comma 1281:1282
Number 1283:1284 text="5" suf="" isf=0 int=5
RParen 1284:1285
RParen 1285:1286
Dot 1286:1287
Ident 1287:1291 text="toBe"
LParen 1291:1292
Minus 1292:1293
Number 1293:1294 text="2" suf="" isf=0 int=2
RParen 1294:1295
Newline 1295:1296
Ident 1300:1306 text="expect"
LParen 1306:1307
Ident 1307:1310 text="max"
LParen 1310:1311
Number 1311:1312 text="3" suf="" isf=0 int=3
Comma 1312:1313
Number 1314:1315 text="7" suf="" isf=0 int=7
RParen 1315:1316
RParen 1316:1317
Dot 1317:1318
Ident 1318:1322 text="toBe"
LParen 1322:1323
Number 1323:1324 text="7" suf="" isf=0 int=7
RParen 1324:1325
Newline 1325:1326
Ident 1330:1336 text="expect"
LParen 1336:1337
Ident 1337:1340 text="max"
LParen 1340:1341
Number 1341:1342 text="7" suf="" isf=0 int=7
Comma 1342:1343
Number 1344:1345 text="3" suf="" isf=0 int=3
RParen 1345:1346
RParen 1346:1347
Dot 1347:1348
Ident 1348:1352 text="toBe"
LParen 1352:1353
Number 1353:1354 text="7" suf="" isf=0 int=7
RParen 1354:1355
Newline 1355:1356
Ident 1360:1366 text="expect"
LParen 1366:1367
Ident 1367:1370 text="max"
LParen 1370:1371
Minus 1371:1372
Number 1372:1373 text="2" suf="" isf=0 int=2
Comma 1373:1374
Minus 1375:1376
Number 1376:1377 text="5" suf="" isf=0 int=5
RParen 1377:1378
RParen 1378:1379
Dot 1379:1380
Ident 1380:1384 text="toBe"
LParen 1384:1385
Minus 1385:1386
Number 1386:1387 text="2" suf="" isf=0 int=2
RParen 1387:1388
Newline 1388:1389
RBrace 1389:1390
Newline 1390:1391
Newline 1391:1392
Ident 1392:1396 text="test"
Fn 1397:1399
Ident 1400:1410 text="min_floats"
LParen 1410:1411
RParen 1411:1412
LBrace 1413:1414
Newline 1414:1415
Ident 1419:1425 text="expect"
LParen 1425:1426
Ident 1426:1429 text="min"
LParen 1429:1430
Number 1430:1433 text="0.5" suf="" isf=1 int=-
Comma 1433:1434
Number 1435:1438 text="1.5" suf="" isf=1 int=-
RParen 1438:1439
RParen 1439:1440
Dot 1440:1441
Ident 1441:1445 text="toBe"
LParen 1445:1446
Number 1446:1449 text="0.5" suf="" isf=1 int=-
RParen 1449:1450
Newline 1450:1451
Ident 1455:1461 text="expect"
LParen 1461:1462
Ident 1462:1465 text="max"
LParen 1465:1466
Number 1466:1469 text="0.5" suf="" isf=1 int=-
Comma 1469:1470
Number 1471:1474 text="1.5" suf="" isf=1 int=-
RParen 1474:1475
RParen 1475:1476
Dot 1476:1477
Ident 1477:1481 text="toBe"
LParen 1481:1482
Number 1482:1485 text="1.5" suf="" isf=1 int=-
RParen 1485:1486
Newline 1486:1487
RBrace 1487:1488
Newline 1488:1489
Newline 1489:1490
Ident 1490:1494 text="test"
Fn 1495:1497
Ident 1498:1508 text="clamp_ints"
LParen 1508:1509
RParen 1509:1510
LBrace 1511:1512
Newline 1512:1513
Ident 1517:1523 text="expect"
LParen 1523:1524
Ident 1524:1529 text="clamp"
LParen 1529:1530
Number 1530:1531 text="5" suf="" isf=0 int=5
Comma 1531:1532
Number 1533:1534 text="0" suf="" isf=0 int=0
Comma 1534:1535
Number 1536:1538 text="10" suf="" isf=0 int=10
RParen 1538:1539
RParen 1539:1540
Dot 1540:1541
Ident 1541:1545 text="toBe"
LParen 1545:1546
Number 1546:1547 text="5" suf="" isf=0 int=5
RParen 1547:1548
Newline 1548:1549
Ident 1553:1559 text="expect"
LParen 1559:1560
Ident 1560:1565 text="clamp"
LParen 1565:1566
Minus 1566:1567
Number 1567:1568 text="3" suf="" isf=0 int=3
Comma 1568:1569
Number 1570:1571 text="0" suf="" isf=0 int=0
Comma 1571:1572
Number 1573:1575 text="10" suf="" isf=0 int=10
RParen 1575:1576
RParen 1576:1577
Dot 1577:1578
Ident 1578:1582 text="toBe"
LParen 1582:1583
Number 1583:1584 text="0" suf="" isf=0 int=0
RParen 1584:1585
Newline 1585:1586
Ident 1590:1596 text="expect"
LParen 1596:1597
Ident 1597:1602 text="clamp"
LParen 1602:1603
Number 1603:1605 text="42" suf="" isf=0 int=42
Comma 1605:1606
Number 1607:1608 text="0" suf="" isf=0 int=0
Comma 1608:1609
Number 1610:1612 text="10" suf="" isf=0 int=10
RParen 1612:1613
RParen 1613:1614
Dot 1614:1615
Ident 1615:1619 text="toBe"
LParen 1619:1620
Number 1620:1622 text="10" suf="" isf=0 int=10
RParen 1622:1623
Newline 1623:1624
RBrace 1624:1625
Newline 1625:1626
Newline 1626:1627
Ident 1627:1631 text="test"
Fn 1632:1634
Ident 1635:1647 text="clamp_floats"
LParen 1647:1648
RParen 1648:1649
LBrace 1650:1651
Newline 1651:1652
Ident 1656:1662 text="expect"
LParen 1662:1663
Ident 1663:1668 text="clamp"
LParen 1668:1669
Number 1669:1672 text="2.5" suf="" isf=1 int=-
Comma 1672:1673
Number 1674:1677 text="0.0" suf="" isf=1 int=-
Comma 1677:1678
Number 1679:1682 text="1.0" suf="" isf=1 int=-
RParen 1682:1683
RParen 1683:1684
Dot 1684:1685
Ident 1685:1689 text="toBe"
LParen 1689:1690
Number 1690:1693 text="1.0" suf="" isf=1 int=-
RParen 1693:1694
Newline 1694:1695
Ident 1699:1705 text="expect"
LParen 1705:1706
Ident 1706:1711 text="clamp"
LParen 1711:1712
Minus 1712:1713
Number 1713:1716 text="2.5" suf="" isf=1 int=-
Comma 1716:1717
Number 1718:1721 text="0.0" suf="" isf=1 int=-
Comma 1721:1722
Number 1723:1726 text="1.0" suf="" isf=1 int=-
RParen 1726:1727
RParen 1727:1728
Dot 1728:1729
Ident 1729:1733 text="toBe"
LParen 1733:1734
Number 1734:1737 text="0.0" suf="" isf=1 int=-
RParen 1737:1738
Newline 1738:1739
Ident 1743:1749 text="expect"
LParen 1749:1750
Ident 1750:1755 text="clamp"
LParen 1755:1756
Number 1756:1759 text="0.5" suf="" isf=1 int=-
Comma 1759:1760
Number 1761:1764 text="0.0" suf="" isf=1 int=-
Comma 1764:1765
Number 1766:1769 text="1.0" suf="" isf=1 int=-
RParen 1769:1770
RParen 1770:1771
Dot 1771:1772
Ident 1772:1776 text="toBe"
LParen 1776:1777
Number 1777:1780 text="0.5" suf="" isf=1 int=-
RParen 1780:1781
Newline 1781:1782
RBrace 1782:1783
Newline 1783:1784
Newline 1784:1785
Ident 1785:1789 text="test"
Fn 1790:1792
Ident 1793:1804 text="str_numbers"
LParen 1804:1805
RParen 1805:1806
LBrace 1807:1808
Newline 1808:1809
Ident 1813:1819 text="expect"
LParen 1819:1820
Ident 1820:1823 text="str"
LParen 1823:1824
Number 1824:1826 text="42" suf="" isf=0 int=42
RParen 1826:1827
RParen 1827:1828
Dot 1828:1829
Ident 1829:1833 text="toBe"
LParen 1833:1834
Str 1834:1838 T("42")
RParen 1838:1839
Newline 1839:1840
Ident 1844:1850 text="expect"
LParen 1850:1851
Ident 1851:1854 text="str"
LParen 1854:1855
Minus 1855:1856
Number 1856:1857 text="7" suf="" isf=0 int=7
RParen 1857:1858
RParen 1858:1859
Dot 1859:1860
Ident 1860:1864 text="toBe"
LParen 1864:1865
Str 1865:1869 T("-7")
RParen 1869:1870
Newline 1870:1871
Ident 1875:1881 text="expect"
LParen 1881:1882
Ident 1882:1885 text="str"
LParen 1885:1886
Number 1886:1889 text="2.5" suf="" isf=1 int=-
RParen 1889:1890
RParen 1890:1891
Dot 1891:1892
Ident 1892:1896 text="toBe"
LParen 1896:1897
Str 1897:1902 T("2.5")
RParen 1902:1903
Newline 1903:1904
Ident 1908:1914 text="expect"
LParen 1914:1915
Ident 1915:1918 text="str"
LParen 1918:1919
True 1919:1923
RParen 1923:1924
RParen 1924:1925
Dot 1925:1926
Ident 1926:1930 text="toBe"
LParen 1930:1931
Str 1931:1937 T("true")
RParen 1937:1938
Newline 1938:1939
RBrace 1939:1940
Newline 1940:1941
Newline 1941:1942
Ident 1942:1946 text="test"
Fn 1947:1949
Ident 1950:1964 text="interp_numbers"
LParen 1964:1965
RParen 1965:1966
LBrace 1967:1968
Newline 1968:1969
Let 1973:1976
Ident 1977:1978 text="s"
Assign 1979:1980
Str 1981:2004 T("x=") E[Number 1985:1987 text="42" suf="" isf=0 int=42; Eof 1988:1988] T(" y=") E[Minus 1992:1993; Number 1993:1994 text="7" suf="" isf=0 int=7; Eof 1995:1995] T(" f=") E[Number 1999:2002 text="2.5" suf="" isf=1 int=-; Eof 2003:2003]
Newline 2004:2005
Ident 2009:2015 text="expect"
LParen 2015:2016
Ident 2016:2017 text="s"
RParen 2017:2018
Dot 2018:2019
Ident 2019:2023 text="toBe"
LParen 2023:2024
Str 2024:2041 T("x=42 y=-7 f=2.5")
RParen 2041:2042
Newline 2042:2043
RBrace 2043:2044
Newline 2044:2045
Newline 2045:2046
Ident 2046:2050 text="test"
Fn 2051:2053
Ident 2054:2081 text="interp_concat_codegen_style"
LParen 2081:2082
RParen 2082:2083
LBrace 2084:2085
Newline 2085:2086
Let 2090:2093
Ident 2094:2097 text="fn_"
Assign 2098:2099
Number 2100:2101 text="3" suf="" isf=0 int=3
Newline 2101:2102
Let 2106:2109
Ident 2110:2112 text="id"
Assign 2113:2114
Number 2115:2116 text="7" suf="" isf=0 int=7
Newline 2116:2117
Let 2121:2124
Ident 2125:2126 text="s"
Assign 2127:2128
Str 2129:2135 T("pkl_")
Plus 2136:2137
Ident 2138:2141 text="str"
LParen 2141:2142
Ident 2142:2145 text="fn_"
RParen 2145:2146
Plus 2147:2148
Str 2149:2152 T("_")
Plus 2153:2154
Ident 2155:2158 text="str"
LParen 2158:2159
Ident 2159:2161 text="id"
RParen 2161:2162
Newline 2162:2163
Ident 2167:2173 text="expect"
LParen 2173:2174
Ident 2174:2175 text="s"
RParen 2175:2176
Dot 2176:2177
Ident 2177:2181 text="toBe"
LParen 2181:2182
Str 2182:2191 T("pkl_3_7")
RParen 2191:2192
Newline 2192:2193
RBrace 2193:2194
Eof 2194:2194
