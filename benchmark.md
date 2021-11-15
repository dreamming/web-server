# WRK测试
docker run --rm  8lovelife/wrk -txxx -cxxx -d30 --timeout=15 http://host.docker.internal:port

## ROUND 1
thread = 10
connection = 10

docker run --rm  8lovelife/wrk -t10 -c10 -d30 --timeout=30 http://host.docker.internal:17878

|-| QPS |THROUGHPUT|AVG |MEMORY | CPU |
|:-----:| :-----:| :----: | :----: |:----: |:----: |
|V1 single-thread       |0.63 | 19| 12.23s  |2 |3  |
|V2 multi-thread        | 6.32 | 190 | 1.51s |2 |3  |
|V3 thread-pool         |6.29 | 189 |1.52s  |2 |3  |
|V4 async function      | 6.32 | 190 | 1.51s |2 |3  |
|V5 async multi-threadd | 6.32 | 190 | 1.51s |2 |3  |

## ROUND 2
thread = 100
connection = 100

docker run --rm  8lovelife/wrk -t100 -c100 -d30 --timeout=30 http://host.docker.internal:17878

|-| QPS |THROUGHPUT|AVG |MEMORY | CPU |
|:-----:| :-----:| :----: | :----: |:----: |:----: |
|V1 single thread       |0.63 | 19| 16.50s  |2 |3  |
|V2 multi-thread        | 63.21 | 1900 | 1.51s |2 |3  |
|V3 thread-pool         |6.61 | 199 |11.67  |2 |3  |
|V4 async function      | 63.22 | 1900 | 1.51 |2 |3  |
|V5 async multi-threadd | 63.18 | 1900 | 1.51 |2 |3  |


## ROUND 3
thread = 1000
connection = 1000

docker run --rm  8lovelife/wrk -t1000 -c1000 -d30 --timeout=30 http://host.docker.internal:17878

|-| QPS |THROUGHPUT|AVG |MEMORY | CPU |
|:-----:| :-----:| :----: | :----: |:----: |:----: |
|V1 single thread       |0.63 | 19| 16.50s  |2 |3  |
|V2 multi thread        | X | X | X |down机 |down机  |
|V3 thread-pool         |6.60 | 199 |15.74s  |2 |3  |
|V4 async function      | 614.44 | 18529 | 1.54s |2 |0  |
|V5 async multi-thread | 334.68 | 10092 | 1.54s |2 |0  |


## ROUND 4
thread = 3000
connection = 3000

docker run --rm  8lovelife/wrk -t3000 -c3000 -d30 --timeout=30 http://host.docker.internal:17878

|-| QPS |THROUGHPUT|AVG |MEMORY | CPU |timeout|
|:-----:| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single thread       |0.73 | 23| 16.50s  |2 |3  |5  |
|V2 multi thread        | X | X | X |down机 |down机  |N/A  |
|V3 thread pool         |6.78 | 209 |13.52s  |2 |3  |40  |
|V4 async function      | 252.81 | 7726 | 6.71s |2 |3  |0  |
|V5 async multi-threadd | 573.04 | 17670 | 3.75s |0|0  |0  |

## ROUND 5
thread = 5000
connection = 5000

docker run --rm  8lovelife/wrk -t5000 -c5000 -d30 --timeout=30 http://host.docker.internal:17878

|-| QPS |THROUGHPUT|AVG |MEMORY | CPU |timeout|
|:-----:| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single thread       |0.73 | 23| 16.50s  |2 |3  |5  |
|V2 multi-thread        | X | X | X |down机 |down机  |N/A  |
|V3 thread-pool         |7.59 | 239 |15.96s  |2 |3  |40  |
|V4 async function      | 433.76 | 14177 | 6.71s |2 |3  |74  |
|V5 async multi-threadd | X | X | X |down机 |down机  |N/A  |