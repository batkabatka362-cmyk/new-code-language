/**
 * ============================================================================
 * CRON Modern C++20 Header-Only RAII Library (cron_rt.hpp)
 * Zero-overhead integration for C++, SAGI Tree Graph, and Game Engines
 * ============================================================================
 */

#ifndef CRON_RT_HPP
#define CRON_RT_HPP

#include "cron_rt.h"
#include <string>
#include <string_view>
#include <vector>
#include <span>
#include <stdexcept>
#include <memory>
#include <array>
#include <optional>

namespace cron {

// ----------------------------------------------------------------------------
// 1. Telemetry Structure
// ----------------------------------------------------------------------------

using Telemetry = cron_telemetry_t;

// ----------------------------------------------------------------------------
// 2. In-Process 256-Core 4D-Torus VM (RAII)
// ----------------------------------------------------------------------------

class VM {
public:
    VM() : vm_(cron_vm_create()) {
        if (!vm_) {
            throw std::runtime_error("Failed to initialize CRON 4D-Torus Virtual Machine.");
        }
    }

    ~VM() {
        if (vm_) {
            cron_vm_destroy(vm_);
            vm_ = nullptr;
        }
    }

    VM(const VM&) = delete;
    VM& operator=(const VM&) = delete;

    VM(VM&& other) noexcept : vm_(other.vm_) {
        other.vm_ = nullptr;
    }

    VM& operator=(VM&& other) noexcept {
        if (this != &other) {
            if (vm_) cron_vm_destroy(vm_);
            vm_ = other.vm_;
            other.vm_ = nullptr;
        }
        return *this;
    }

    void reset() {
        if (vm_) cron_vm_reset(vm_);
    }

    Telemetry run(std::string_view cr_source) {
        std::string null_terminated(cr_source);
        Telemetry t{};
        bool ok = cron_vm_run_cr(vm_, null_terminated.c_str(), &t);
        if (!ok) {
            throw std::runtime_error("CRON .cr execution failed on 4D-Torus simulator.");
        }
        return t;
    }

    Telemetry run_cl(std::string_view cl_code) {
        std::string null_terminated(cl_code);
        Telemetry t{};
        bool ok = cron_vm_run_cl(vm_, null_terminated.c_str(), &t);
        if (!ok) {
            throw std::runtime_error("CRON .cl VLIW execution failed on 4D-Torus simulator.");
        }
        return t;
    }

    cron_vm_t* raw_handle() const { return vm_; }

private:
    cron_vm_t* vm_{nullptr};
};

// ----------------------------------------------------------------------------
// 3. Compiler In-Memory Driver
// ----------------------------------------------------------------------------

class Compiler {
public:
    static std::string to_vliw(std::string_view source) {
        std::string src(source);
        char* err = nullptr;
        char* out = cron_compile_source(src.c_str(), &err);
        if (!out) {
            std::string msg = err ? err : "CRON compilation to VLIW failed";
            if (err) cron_string_free(err);
            throw std::runtime_error(msg);
        }
        std::string res(out);
        cron_string_free(out);
        return res;
    }

    static std::string to_c23(std::string_view source) {
        std::string src(source);
        char* err = nullptr;
        char* out = cron_compile_to_c23(src.c_str(), &err);
        if (!out) {
            std::string msg = err ? err : "CRON compilation to C23 failed";
            if (err) cron_string_free(err);
            throw std::runtime_error(msg);
        }
        std::string res(out);
        cron_string_free(out);
        return res;
    }

    static std::string to_llvm(std::string_view source) {
        std::string src(source);
        char* err = nullptr;
        char* out = cron_compile_to_llvm(src.c_str(), &err);
        if (!out) {
            std::string msg = err ? err : "CRON compilation to LLVM failed";
            if (err) cron_string_free(err);
            throw std::runtime_error(msg);
        }
        std::string res(out);
        cron_string_free(out);
        return res;
    }

    static bool check(std::string_view source, std::string* out_err = nullptr) {
        std::string src(source);
        char* err = nullptr;
        bool ok = cron_check_syntax(src.c_str(), &err);
        if (!ok && out_err && err) {
            *out_err = err;
        }
        if (err) cron_string_free(err);
        return ok;
    }

    static int64_t jit(std::string_view source) {
        std::string src(source);
        char* err = nullptr;
        int64_t res = cron_jit_execute(src.c_str(), &err);
        if (err) {
            std::string msg(err);
            cron_string_free(err);
            throw std::runtime_error(msg);
        }
        return res;
    }

    static std::string version() {
        return cron_version();
    }

    static std::string target_info() {
        return cron_target_info();
    }
};

// ----------------------------------------------------------------------------
// 4. Zero-Copy 4D Tensor Template
// ----------------------------------------------------------------------------

template<typename T>
struct DTypeHelper;

template<> struct DTypeHelper<float>   { static constexpr int32_t type_id = CRON_DTYPE_F32; };
template<> struct DTypeHelper<double>  { static constexpr int32_t type_id = CRON_DTYPE_F64; };
template<> struct DTypeHelper<int32_t> { static constexpr int32_t type_id = CRON_DTYPE_I32; };
template<> struct DTypeHelper<int64_t> { static constexpr int32_t type_id = CRON_DTYPE_I64; };

template<typename T>
class Tensor4D {
public:
    Tensor4D(int32_t n, int32_t c, int32_t h, int32_t w)
        : raw_(cron_tensor4d_create(n, c, h, w, DTypeHelper<T>::type_id)) {
        if (!raw_) {
            throw std::bad_alloc();
        }
    }

    ~Tensor4D() {
        if (raw_) {
            cron_tensor4d_destroy(raw_);
            raw_ = nullptr;
        }
    }

    Tensor4D(const Tensor4D&) = delete;
    Tensor4D& operator=(const Tensor4D&) = delete;

    Tensor4D(Tensor4D&& other) noexcept : raw_(other.raw_) {
        other.raw_ = nullptr;
    }

    Tensor4D& operator=(Tensor4D&& other) noexcept {
        if (this != &other) {
            if (raw_) cron_tensor4d_destroy(raw_);
            raw_ = other.raw_;
            other.raw_ = nullptr;
        }
        return *this;
    }

    T* data() noexcept {
        return reinterpret_cast<T*>(cron_tensor4d_data_ptr(raw_));
    }

    const T* data() const noexcept {
        return reinterpret_cast<const T*>(cron_tensor4d_data_ptr(raw_));
    }

    std::span<T> span() noexcept {
        return std::span<T>(data(), raw_->total_elements);
    }

    std::span<const T> span() const noexcept {
        return std::span<const T>(data(), raw_->total_elements);
    }

    std::array<int32_t, 4> shape() const noexcept {
        return {raw_->shape[0], raw_->shape[1], raw_->shape[2], raw_->shape[3]};
    }

    size_t total_elements() const noexcept {
        return raw_->total_elements;
    }

    size_t size_bytes() const noexcept {
        return raw_->size_bytes;
    }

    T& operator()(int32_t n, int32_t c, int32_t h, int32_t w) {
        size_t idx = static_cast<size_t>(n * raw_->strides[0] +
                                         c * raw_->strides[1] +
                                         h * raw_->strides[2] +
                                         w * raw_->strides[3]);
        return data()[idx];
    }

    const T& operator()(int32_t n, int32_t c, int32_t h, int32_t w) const {
        size_t idx = static_cast<size_t>(n * raw_->strides[0] +
                                         c * raw_->strides[1] +
                                         h * raw_->strides[2] +
                                         w * raw_->strides[3]);
        return data()[idx];
    }

    cron_tensor4d_t* raw_handle() const noexcept { return raw_; }

private:
    cron_tensor4d_t* raw_{nullptr};
};

// ----------------------------------------------------------------------------
// 5. Photonic Accelerator Bridge
// ----------------------------------------------------------------------------

class PhotonicEngine {
public:
    static void mzi_gemm(
        std::span<const double> amps,
        std::span<const double> phases,
        std::span<double> out_amps,
        std::span<double> out_phases
    ) {
        if (amps.size() != phases.size() || amps.size() != out_amps.size() || amps.size() != out_phases.size()) {
            throw std::invalid_argument("Tensor span lengths must match for MZI GEMM.");
        }
        cron_photonic_mzi_gemm(
            amps.data(),
            phases.data(),
            amps.size(),
            out_amps.data(),
            out_phases.data()
        );
    }
};

// ----------------------------------------------------------------------------
// 6. CSP Typed Channels & 4D-Torus Concurrency Bridge
// ----------------------------------------------------------------------------

template<typename T = int64_t>
class Channel {
public:
    explicit Channel(uint32_t capacity = 64)
        : id_(cron_channel_create_c(capacity)) {}

    ~Channel() {
        if (id_ != 0) {
            cron_channel_close_c(id_);
            id_ = 0;
        }
    }

    Channel(const Channel&) = delete;
    Channel& operator=(const Channel&) = delete;

    Channel(Channel&& other) noexcept : id_(other.id_) {
        other.id_ = 0;
    }

    Channel& operator=(Channel&& other) noexcept {
        if (this != &other) {
            if (id_ != 0) cron_channel_close_c(id_);
            id_ = other.id_;
            other.id_ = 0;
        }
        return *this;
    }

    bool send(T val) {
        return cron_channel_send_c(id_, static_cast<int64_t>(val)) == 0;
    }

    T recv() {
        return static_cast<T>(cron_channel_recv_c(id_));
    }

    std::optional<T> try_recv() {
        int64_t res = cron_channel_try_recv_c(id_);
        if (res == -1) {
            return std::nullopt;
        }
        return static_cast<T>(res);
    }

    void close() {
        if (id_ != 0) {
            cron_channel_close_c(id_);
            id_ = 0;
        }
    }

    uint64_t id() const noexcept { return id_; }

private:
    uint64_t id_{0};
};

class TorusMesh {
public:
    static int64_t distance(int64_t core_a, int64_t core_b) {
        return cron_torus_distance_c(core_a, core_b);
    }
};

} // namespace cron

#endif /* CRON_RT_HPP */

