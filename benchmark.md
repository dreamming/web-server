# ROUND 1

```
线程数:10
HTTP链接数:10
```

|-| QPS |THROUGHPUT|AVG |MEMORY QUOTA | CPU QUOTA | TIMEOUT|
|:-----| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single-thread       |0.63  | 19  |12.23s |10M|2 |0  |
|V2 multi-thread        | 6.32 | 190 | 1.51s |10M|2 |0  |
|V3 thread-pool         |6.29  | 189 |1.52s  |10M|2 |0  |
|V4 async function      | 6.32 | 190 | 1.51s |10M|2 |0  |
|V5 async multi-thread  | 6.32 | 190 | 1.51s |10M|2 |0  |

# ROUND 2
```
线程数:100
HTTP链接数:100
```

|-| QPS |THROUGHPUT|AVG |MEMORY QUOTA | CPU QUOTA |TIMEOUT|
|:-----| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single-thread       |0.63   | 19   | 16.50s  |10M|2 |0  |
|V2 multi-thread        | 63.21 | 1900 | 1.51s   |10M|2 |0  |
|V3 thread-pool         |6.61   | 199  |11.67    |10M|2 |0  |
|V4 async function      | 63.22 | 1900 | 1.51    |10M|2 |0  |
|V5 async multi-thread  | 63.18 | 1900 | 1.51    |10M|2 |0  |


# ROUND 3
```
线程数:1000
HTTP链接数:1000
```

|-| QPS |THROUGHPUT|AVG |MEMORY QUOTA | CPU QUOTA |TIMEOUT|
|:-----| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single-thread       |0.63    | 19    |16.50s  |10M |2  |0  |
|V2 multi-thread        |NA      |N/A    |N/A     |10M |2  |N/A|
|V3 thread-pool         |6.60    | 199   |15.74s  |10M |2  |0  |
|V4 async function      | 614.44 | 18529 | 1.54s  |10M |2  |0  |
|V5 async multi-thread  | 615.77  | 18565 | 1.54s  |10M |2  |0  |


# ROUND 4
```
线程数:3000
HTTP链接数:3000
```

|-| QPS |THROUGHPUT|AVG |MEMORY QUOTA | CPU QUOTA |TIMEOUT|
|:-----| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single-thread       |0.73    | 23    | 16.50s  |10M |2  |5  |
|V2 multi-thread        |NA      |N/A    |N/A      |10M |2  |N/A|
|V3 thread-pool         |6.78    | 209   |13.52s   |10M |2  |10 |
|V4 async function      | 673.95 | 20734 | 2.72s   |10M |2  |0  |
|V5 async multi-thread  | 635.85 | 19738 | 3.49s   |10M |2  |0  |

# ROUND 5
```
线程数:5000
HTTP链接数:5000
```

|-| QPS |THROUGHPUT|AVG |MEMORY QUOTA | CPU QUOTA |TIMEOUT|
|:-----| :-----:| :----: | :----: |:----: |:----: |:----: |
|V1 single thread       |0.73    | 23    | 16.50s |10M |2  |5  |
|V2 multi-thread        | N/A    | N/A   | N/A    |10M |2  |N/A|
|V3 thread-pool         |7.59    | 239   |15.96s  |10M |2  |40 |
|V4 async function      | 593.99 | 20044 | 4.64s  |10M |2  |0  |
|V4 async function      | 911.34 | 28938 | 2.15s  |20M |2  |0  |
|V5 async multi-thread  | N/A    | N/A   | N/A    |10M |2  |N/A|
|V5 async multi-thread  | 896.92 | 28978 | 2.42s  |20M |2  |0  |