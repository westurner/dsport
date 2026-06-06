FROM gcr.io/oss-fuzz-base/base-builder-rust

# Set up sanitizer environment variables for OSS-Fuzz
# These control how sanitizers behave during fuzzing

# Address Sanitizer Options
ENV ASAN_OPTIONS="\
detect_leaks=1:\
detect_odr_violation=1:\
detect_stack_use_after_return=1:\
handle_abort=1:\
halt_on_error=0"

# Memory Sanitizer Options
ENV MSAN_OPTIONS="\
track_origins=2:\
halt_on_error=0"

# Undefined Behavior Sanitizer Options
ENV UBSAN_OPTIONS="\
halt_on_error=0:\
print_stacktrace=1"

# Fuzzer options (common for all targets)
ENV FUZZER_OPTIONS="\
-max_len=8192:\
-timeout=10:\
-artifact_prefix=crashes/:\
-max_total_time=0"

# Copy project
COPY . $SRC/dsport
WORKDIR $SRC/dsport/src/pygmentsrs/fuzz

# Copy build script
COPY oss-fuzz/build.sh $SRC/
RUN chmod +x $SRC/build.sh

