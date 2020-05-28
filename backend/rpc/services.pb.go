// Code generated by protoc-gen-go. DO NOT EDIT.
// source: services.proto

package rpc

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type Strategy int32

const (
	Strategy_Trailing Strategy = 0
)

var Strategy_name = map[int32]string{
	0: "Trailing",
}

var Strategy_value = map[string]int32{
	"Trailing": 0,
}

func (x Strategy) String() string {
	return proto.EnumName(Strategy_name, int32(x))
}

func (Strategy) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_8e16ccb8c5307b32, []int{0}
}

type BotInfo struct {
	Id                   string   `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Strategy             Strategy `protobuf:"varint,2,opt,name=strategy,proto3,enum=services.Strategy" json:"strategy,omitempty"`
	Name                 string   `protobuf:"bytes,3,opt,name=name,proto3" json:"name,omitempty"`
	Config               string   `protobuf:"bytes,4,opt,name=config,proto3" json:"config,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *BotInfo) Reset()         { *m = BotInfo{} }
func (m *BotInfo) String() string { return proto.CompactTextString(m) }
func (*BotInfo) ProtoMessage()    {}
func (*BotInfo) Descriptor() ([]byte, []int) {
	return fileDescriptor_8e16ccb8c5307b32, []int{0}
}

func (m *BotInfo) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_BotInfo.Unmarshal(m, b)
}
func (m *BotInfo) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_BotInfo.Marshal(b, m, deterministic)
}
func (m *BotInfo) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BotInfo.Merge(m, src)
}
func (m *BotInfo) XXX_Size() int {
	return xxx_messageInfo_BotInfo.Size(m)
}
func (m *BotInfo) XXX_DiscardUnknown() {
	xxx_messageInfo_BotInfo.DiscardUnknown(m)
}

var xxx_messageInfo_BotInfo proto.InternalMessageInfo

func (m *BotInfo) GetId() string {
	if m != nil {
		return m.Id
	}
	return ""
}

func (m *BotInfo) GetStrategy() Strategy {
	if m != nil {
		return m.Strategy
	}
	return Strategy_Trailing
}

func (m *BotInfo) GetName() string {
	if m != nil {
		return m.Name
	}
	return ""
}

func (m *BotInfo) GetConfig() string {
	if m != nil {
		return m.Config
	}
	return ""
}

type BotInfoList struct {
	Bots                 []*BotInfo `protobuf:"bytes,1,rep,name=bots,proto3" json:"bots,omitempty"`
	XXX_NoUnkeyedLiteral struct{}   `json:"-"`
	XXX_unrecognized     []byte     `json:"-"`
	XXX_sizecache        int32      `json:"-"`
}

func (m *BotInfoList) Reset()         { *m = BotInfoList{} }
func (m *BotInfoList) String() string { return proto.CompactTextString(m) }
func (*BotInfoList) ProtoMessage()    {}
func (*BotInfoList) Descriptor() ([]byte, []int) {
	return fileDescriptor_8e16ccb8c5307b32, []int{1}
}

func (m *BotInfoList) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_BotInfoList.Unmarshal(m, b)
}
func (m *BotInfoList) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_BotInfoList.Marshal(b, m, deterministic)
}
func (m *BotInfoList) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BotInfoList.Merge(m, src)
}
func (m *BotInfoList) XXX_Size() int {
	return xxx_messageInfo_BotInfoList.Size(m)
}
func (m *BotInfoList) XXX_DiscardUnknown() {
	xxx_messageInfo_BotInfoList.DiscardUnknown(m)
}

var xxx_messageInfo_BotInfoList proto.InternalMessageInfo

func (m *BotInfoList) GetBots() []*BotInfo {
	if m != nil {
		return m.Bots
	}
	return nil
}

type BotInfoListRequest struct {
	Offset               int64    `protobuf:"varint,1,opt,name=offset,proto3" json:"offset,omitempty"`
	Limit                int64    `protobuf:"varint,2,opt,name=limit,proto3" json:"limit,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *BotInfoListRequest) Reset()         { *m = BotInfoListRequest{} }
func (m *BotInfoListRequest) String() string { return proto.CompactTextString(m) }
func (*BotInfoListRequest) ProtoMessage()    {}
func (*BotInfoListRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_8e16ccb8c5307b32, []int{2}
}

func (m *BotInfoListRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_BotInfoListRequest.Unmarshal(m, b)
}
func (m *BotInfoListRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_BotInfoListRequest.Marshal(b, m, deterministic)
}
func (m *BotInfoListRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BotInfoListRequest.Merge(m, src)
}
func (m *BotInfoListRequest) XXX_Size() int {
	return xxx_messageInfo_BotInfoListRequest.Size(m)
}
func (m *BotInfoListRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_BotInfoListRequest.DiscardUnknown(m)
}

var xxx_messageInfo_BotInfoListRequest proto.InternalMessageInfo

func (m *BotInfoListRequest) GetOffset() int64 {
	if m != nil {
		return m.Offset
	}
	return 0
}

func (m *BotInfoListRequest) GetLimit() int64 {
	if m != nil {
		return m.Limit
	}
	return 0
}

func init() {
	proto.RegisterEnum("services.Strategy", Strategy_name, Strategy_value)
	proto.RegisterType((*BotInfo)(nil), "services.BotInfo")
	proto.RegisterType((*BotInfoList)(nil), "services.BotInfoList")
	proto.RegisterType((*BotInfoListRequest)(nil), "services.BotInfoListRequest")
}

func init() { proto.RegisterFile("services.proto", fileDescriptor_8e16ccb8c5307b32) }

var fileDescriptor_8e16ccb8c5307b32 = []byte{
	// 258 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x6c, 0x90, 0x4f, 0x4b, 0xc4, 0x30,
	0x10, 0xc5, 0xed, 0x9f, 0x5d, 0xeb, 0x54, 0x8a, 0x0e, 0x2a, 0x41, 0x3c, 0x94, 0x82, 0x50, 0x3c,
	0xf4, 0x50, 0xfd, 0x04, 0xbd, 0x09, 0x0a, 0x12, 0x3d, 0x79, 0xcb, 0x76, 0xd3, 0x12, 0xd8, 0x4d,
	0xd6, 0x64, 0x56, 0xf0, 0xdb, 0xcb, 0xc6, 0x74, 0x5d, 0xd0, 0x5b, 0x5e, 0xde, 0xcc, 0xbc, 0x1f,
	0x0f, 0x0a, 0x27, 0xed, 0xa7, 0xea, 0xa5, 0x6b, 0x36, 0xd6, 0x90, 0xc1, 0x6c, 0xd2, 0xd5, 0x16,
	0x8e, 0x3b, 0x43, 0x8f, 0x7a, 0x30, 0x58, 0x40, 0xac, 0x96, 0x2c, 0x2a, 0xa3, 0xfa, 0x84, 0xc7,
	0x6a, 0x89, 0x0d, 0x64, 0x8e, 0xac, 0x20, 0x39, 0x7e, 0xb1, 0xb8, 0x8c, 0xea, 0xa2, 0xc5, 0x66,
	0x7f, 0xe7, 0x35, 0x38, 0x7c, 0x3f, 0x83, 0x08, 0xa9, 0x16, 0x6b, 0xc9, 0x12, 0x7f, 0xc1, 0xbf,
	0xf1, 0x0a, 0xe6, 0xbd, 0xd1, 0x83, 0x1a, 0x59, 0xea, 0x7f, 0x83, 0xaa, 0x1e, 0x20, 0x0f, 0xb1,
	0x4f, 0xca, 0x11, 0xde, 0x42, 0xba, 0x30, 0xe4, 0x58, 0x54, 0x26, 0x75, 0xde, 0x9e, 0xff, 0xc6,
	0x84, 0x21, 0xee, 0xed, 0xaa, 0x03, 0x3c, 0xd8, 0xe2, 0xf2, 0x63, 0x2b, 0x1d, 0xed, 0x32, 0xcc,
	0x30, 0x38, 0x49, 0x9e, 0x3d, 0xe1, 0x41, 0xe1, 0x05, 0xcc, 0x56, 0x6a, 0xad, 0xc8, 0xc3, 0x27,
	0xfc, 0x47, 0xdc, 0x31, 0xc8, 0x26, 0x76, 0x3c, 0x85, 0xec, 0xcd, 0x0a, 0xb5, 0x52, 0x7a, 0x3c,
	0x3b, 0x6a, 0x5f, 0x00, 0x3a, 0x43, 0xcf, 0x42, 0x8b, 0x51, 0x5a, 0xec, 0x20, 0xdf, 0x85, 0x4c,
	0xe5, 0xdc, 0xfc, 0x61, 0x3a, 0x40, 0xb8, 0xbe, 0xfc, 0xd7, 0xed, 0x66, 0xef, 0x89, 0xdd, 0xf4,
	0x8b, 0xb9, 0x2f, 0xfd, 0xfe, 0x3b, 0x00, 0x00, 0xff, 0xff, 0x01, 0xfa, 0x7b, 0xee, 0x86, 0x01,
	0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// BotManagerClient is the client API for BotManager service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type BotManagerClient interface {
	ListBotInfo(ctx context.Context, in *BotInfoListRequest, opts ...grpc.CallOption) (*BotInfoList, error)
}

type botManagerClient struct {
	cc *grpc.ClientConn
}

func NewBotManagerClient(cc *grpc.ClientConn) BotManagerClient {
	return &botManagerClient{cc}
}

func (c *botManagerClient) ListBotInfo(ctx context.Context, in *BotInfoListRequest, opts ...grpc.CallOption) (*BotInfoList, error) {
	out := new(BotInfoList)
	err := c.cc.Invoke(ctx, "/services.BotManager/ListBotInfo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// BotManagerServer is the server API for BotManager service.
type BotManagerServer interface {
	ListBotInfo(context.Context, *BotInfoListRequest) (*BotInfoList, error)
}

// UnimplementedBotManagerServer can be embedded to have forward compatible implementations.
type UnimplementedBotManagerServer struct {
}

func (*UnimplementedBotManagerServer) ListBotInfo(ctx context.Context, req *BotInfoListRequest) (*BotInfoList, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListBotInfo not implemented")
}

func RegisterBotManagerServer(s *grpc.Server, srv BotManagerServer) {
	s.RegisterService(&_BotManager_serviceDesc, srv)
}

func _BotManager_ListBotInfo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(BotInfoListRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(BotManagerServer).ListBotInfo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/services.BotManager/ListBotInfo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(BotManagerServer).ListBotInfo(ctx, req.(*BotInfoListRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _BotManager_serviceDesc = grpc.ServiceDesc{
	ServiceName: "services.BotManager",
	HandlerType: (*BotManagerServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "ListBotInfo",
			Handler:    _BotManager_ListBotInfo_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "services.proto",
}
