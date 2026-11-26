.section .text

.global read_cntpct_el0
read_cntpct_el0:
  mrs x0, cntpct_el0
  ret

.global read_cntp_ctl_el0
read_cntp_ctl_el0:
  mrs x0, cntp_ctl_el0
  ret

.global write_cntp_ctl_el0
write_cntp_ctl_el0:
  msr cntp_ctl_el0, x0
  ret

.global write_cntp_cval_el0
write_cntp_cval_el0:
  msr cntp_cval_el0, x0
  ret
