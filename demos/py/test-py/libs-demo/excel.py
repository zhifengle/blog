# https://zhuanlan.zhihu.com/p/237583143
import xlwings as xw

app = xw.App(visible=True, add_book=False)
app.display_alerts = False    # 关闭一些提示信息，可以加快运行速度。 默认为 True。
app.screen_updating = True    # 更新显示工作表的内容。默认为 True。关闭它也可以提升运行速度。
wb = app.books.add()
sht = wb.sheets.active

'''工作簿'''
# wb = app.books.add()                   # 新建工作簿。
# wb = app.books.open(r'file_path')      # 打开现有的工作簿
# wb = app.books.active                  # 获取当前活动的工作簿

"""工作表"""
# sht = wb.sheets.active                 # 获取当前活动的工作表
# sht = wb.sheets[0]                     # 按索引获取工作表
# sht = wb.sheets['Sheet1']              # 按表名获取工作表
# sht1 = wb.sheets.add()                 # 新建工作表，默认新建的放在最前面。
# sht1 = wb.sheets.add('新建工作表', after=sht)   # 新建工作表，放在sht工作表后面。

""" 读取单元格 """
b3 = sht.range('b3')
# 获取 b3 中的值
v = b3.value
# 也可以根据行列号读取
b3_value = sht.range(3, 2).value
# 读取一段区间内的值
a1_c4_value = sht.range('a1:c4').options(
    ndim=2).value       # 加上 option 读取二维的数据
a1_c4_value = sht.range((1, 1), (4, 3)).options(ndim=2).value   # 和上面读取的内容一样。

""" 写入 就是把值赋值给读取的单元格就可以了"""
sht.range(3, 2).value = 'b3'

"""设置单元格大小"""
sht.autofit()    # 自动调整单元格大小。注：此方法是在单元格写入内容后，再使用，才有效。
sht.range(1, 4).column_width = 5    # 设置第4列 列宽。（1,4）为第1行第4列的单元格
sht.range(1, 4).row_height = 20     # 设置第1行 行高

"""设置单元格 字体格式"""
b3.color = 255, 200, 255         # 设置单元格的填充颜色
b3.api.Font.ColorIndex = 3     # 设置字体的颜色，具体颜色索引见下方。
b3.api.Font.Size = 24          # 设置字体的大小。
b3.api.Font.Bold = True        # 设置为粗体。
b3.api.HorizontalAlignment = -4108    # -4108 水平居中。 -4131 靠左，-4152 靠右。
# -4108 垂直居中（默认）。 -4160 靠上，-4107 靠下， -4130 自动换行对齐。
b3.api.VerticalAlignment = -4130
b3.api.NumberFormat = "0.00"          # 设置单元格的数字格式。

"""设置边框"""
# Borders(9) 底部边框，LineStyle = 1 直线。
b3.api.Borders(9).LineStyle = 1
b3.api.Borders(9).Weight = 3                # 设置边框粗细。

# Borders(7) 左边框，LineStyle = 2 虚线。
b3.api.Borders(7).LineStyle = 2
b3.api.Borders(7).Weight = 3

# Borders(8) 顶部框，LineStyle = 5 双点划线。
b3.api.Borders(8).LineStyle = 5
b3.api.Borders(8).Weight = 3

# Borders(10) 右边框，LineStyle = 4 点划线。
b3.api.Borders(10).LineStyle = 4
b3.api.Borders(10).Weight = 3

# Borders(5) 单元格内从左上角 到 右下角。
b3.api.Borders(5).LineStyle = 1
b3.api.Borders(5).Weight = 3

# Borders(6) 单元格内从左下角 到 右上角。
b3.api.Borders(6).LineStyle = 1
b3.api.Borders(6).Weight = 3

"""如果是一个区域的单元格，内部边框设置如下"""
# # Borders(11) 内部垂直边线。
# b3.api.Borders(11).LineStyle = 1
# b3.api.Borders(11).Weight = 3
#
# # Borders(12) 内部水平边线。
# b3.api.Borders(12).LineStyle = 1
# b3.api.Borders(12).Weight = 3

"""合并拆分单元格"""
sht.range('C8:D8').api.merge()      # 合并单元格 C8 到 D8
sht.range('C8:D8').api.unmerge()    # 拆分单元格。

'''插入 、删除 一行'''
sht1.range('a3').api.EntireRow.Delete()     # 会删除 ’a3‘ 单元格所在的行。
sht1.api.Rows(3).Insert()                   # 会在第3行插入一行，原来的第3行下移。

'''插入 、删除 一列'''
sht1.range('c2').api.EntireColumn.Delete()  # 会删除 ’c2‘ 单元格所在的列。
sht1.api.Columns(3).Insert()                # 会在第3列插入一列，原来的第3列右移。(也可以用列的字母表示)

'''选择sheet页面最右下角的单元格，获取最大行数，和列数'''
# 区别 expand(), expand()只选中与之连续的单元格。
cell = sht1.used_range.last_cell
rows = cell.row
columns = cell.column

# cell = sht1.range("a1").expand("down")
# max_rows = cell.rows.count              # 获取最大行数


'''排序，删除重复值'''
# 排序使用方法：
# 1、选择需要排序的区域。这里用 'a2' 是因为排序的数据送从第二行开始的，第一行是标题，不应该参与排序。
# 2、选择按那一列进行排序 Key1=sht.range('c2').api， 这里选择的是按 第 C 列排序，所以这里选择 c1 和 c2 都可以。
# 3、Order1=1 为升序，2为降序。
sht1.range('a2', (rows, columns)).api.Sort(Key1=sht.range('c2').api, Order1=1)

# 删除重复值使用方法：
# RemoveDuplicates(3) 为按第3列内容进行删除重复项。
sht1.range('a2', (rows, columns)).api.RemoveDuplicates(3)


"""插入、读取公式"""
sht1.range('d1').formula = '=sum(e1+f1)'    # 插入公式
print(sht1.range('d1').formula)

'''同个表格复制、粘贴'''
# 复制 a2 到 a6 之间单元格的值，粘贴到'a15'中
sht.range('a2', 'a6').api.Copy(sht.range('a15').api)

'''跨表格复制、粘贴'''
my_values = sht_1.range('a2：d4').options(ndim=2).value    # 读取二维的数据
sht_2.range('a1').value = my_values

wb.save()
# wb.close()
# app.quit()
